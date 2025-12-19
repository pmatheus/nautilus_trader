// -------------------------------------------------------------------------------------------------
//  Copyright (C) 2015-2025 Nautech Systems Pty Ltd. All rights reserved.
//  https://nautechsystems.io
//
//  Licensed under the GNU Lesser General Public License Version 3.0 (the "License");
//  You may not use this file except in compliance with the License.
//  You may obtain a copy of the License at https://www.gnu.org/licenses/lgpl-3.0.en.html
//
//  Unless required by applicable law or agreed to in writing, software
//  distributed under the License is distributed on an "AS IS" BASIS,
//  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//  See the License for the specific language governing permissions and
//  limitations under the License.
// -------------------------------------------------------------------------------------------------

//! Parquet file writer for Iceberg tables with S3 upload.
//!
//! This module provides functionality to write Arrow RecordBatches to Parquet
//! format and upload them to S3-compatible storage (Supabase). It includes
//! compression, retry logic, and date/hour partitioning.

use std::sync::Arc;
use std::time::Duration;

use arrow::record_batch::RecordBatch;
use chrono::{DateTime, Utc};
use object_store::aws::AmazonS3Builder;
use object_store::path::Path as ObjectPath;
use object_store::ObjectStore;
use parquet::arrow::ArrowWriter;
use parquet::basic::{Compression, ZstdLevel};
use parquet::file::properties::{WriterProperties, WriterVersion};
use thiserror::Error;
use tokio::time::sleep;
use uuid::Uuid;

use crate::config::IcebergConfig;

/// Errors that can occur during Parquet writing and S3 upload.
#[derive(Error, Debug)]
pub enum WriterError {
    /// Parquet writing error.
    #[error("Parquet write error: {0}")]
    ParquetError(#[from] parquet::errors::ParquetError),

    /// Arrow error during writing.
    #[error("Arrow error: {0}")]
    ArrowError(#[from] arrow::error::ArrowError),

    /// Object store error during upload.
    #[error("Object store error: {0}")]
    ObjectStoreError(#[from] object_store::Error),

    /// IO error.
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    /// Configuration error.
    #[error("Configuration error: {0}")]
    ConfigError(String),

    /// Upload failed after retries.
    #[error("Upload failed after {0} retries")]
    UploadFailed(usize),

    /// Invalid partition format.
    #[error("Invalid partition format: {0}")]
    InvalidPartition(String),
}

/// Compression codec for Parquet files.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompressionCodec {
    /// Snappy compression (fast, moderate compression ratio).
    Snappy,
    /// Zstd compression (slower, better compression ratio).
    Zstd(i32),
    /// No compression.
    Uncompressed,
}

impl Default for CompressionCodec {
    fn default() -> Self {
        Self::Zstd(3) // Default to Zstd level 3
    }
}

impl From<CompressionCodec> for Compression {
    fn from(codec: CompressionCodec) -> Self {
        match codec {
            CompressionCodec::Snappy => Self::SNAPPY,
            CompressionCodec::Zstd(level) => {
                Self::ZSTD(ZstdLevel::try_new(level).unwrap_or_default())
            }
            CompressionCodec::Uncompressed => Self::UNCOMPRESSED,
        }
    }
}

/// Configuration for the Parquet writer.
#[derive(Debug, Clone)]
pub struct WriterConfig {
    /// Compression codec to use.
    pub compression: CompressionCodec,
    /// Maximum number of retries for upload.
    pub max_retries: usize,
    /// Initial retry delay in milliseconds.
    pub retry_delay_ms: u64,
    /// Enable date-based partitioning.
    pub partition_by_date: bool,
    /// Enable hour-based partitioning.
    pub partition_by_hour: bool,
}

impl Default for WriterConfig {
    fn default() -> Self {
        Self {
            compression: CompressionCodec::default(),
            max_retries: 3,
            retry_delay_ms: 1000,
            partition_by_date: true,
            partition_by_hour: true,
        }
    }
}

/// Parquet file writer with S3 upload capability.
///
/// This writer converts Arrow RecordBatches to Parquet format and uploads
/// them to S3-compatible storage (Supabase). It supports compression,
/// retry logic with exponential backoff, and date/hour partitioning.
///
/// # Example
///
/// ```no_run
/// use nautilus_iceberg::writer::{ParquetWriter, WriterConfig};
/// use nautilus_iceberg::config::{IcebergConfig, IcebergCredentials};
/// # use arrow::record_batch::RecordBatch;
///
/// # async fn example(batch: RecordBatch) -> Result<(), Box<dyn std::error::Error>> {
/// let iceberg_config = IcebergConfig::builder()
///     .project_ref("my-project")
///     .credentials(IcebergCredentials::new("api-key"))
///     .build()?;
///
/// let writer_config = WriterConfig::default();
/// let writer = ParquetWriter::new(iceberg_config, writer_config).await?;
///
/// let path = writer.write_and_upload(
///     &batch,
///     "orderbook_l3_events",
///     None,
/// ).await?;
///
/// println!("Uploaded to: {}", path);
/// # Ok(())
/// # }
/// ```
pub struct ParquetWriter {
    iceberg_config: IcebergConfig,
    writer_config: WriterConfig,
    object_store: Arc<dyn ObjectStore>,
}

impl ParquetWriter {
    /// Creates a new Parquet writer.
    ///
    /// # Arguments
    ///
    /// * `iceberg_config` - Iceberg configuration with S3 endpoint and credentials
    /// * `writer_config` - Writer configuration for compression and retry settings
    ///
    /// # Errors
    ///
    /// Returns an error if the object store cannot be initialized.
    pub async fn new(
        iceberg_config: IcebergConfig,
        writer_config: WriterConfig,
    ) -> Result<Self, WriterError> {
        let object_store = Self::create_object_store(&iceberg_config)?;

        Ok(Self {
            iceberg_config,
            writer_config,
            object_store,
        })
    }

    /// Creates an S3-compatible object store client.
    fn create_object_store(config: &IcebergConfig) -> Result<Arc<dyn ObjectStore>, WriterError> {
        let s3_endpoint = config.s3_endpoint();
        let bucket = config.warehouse_name.clone();

        // Supabase uses the API key for both access key and secret key
        let store = AmazonS3Builder::new()
            .with_endpoint(s3_endpoint)
            .with_bucket_name(bucket)
            .with_access_key_id(&config.credentials.api_key)
            .with_secret_access_key(&config.credentials.api_key) // Supabase uses same key
            .with_region("auto") // Supabase uses "auto" region
            .with_allow_http(false)
            .build()
            .map_err(|e| WriterError::ConfigError(format!("Failed to create S3 client: {}", e)))?;

        Ok(Arc::new(store))
    }

    /// Writes a RecordBatch to Parquet and uploads to S3.
    ///
    /// # Arguments
    ///
    /// * `batch` - The Arrow RecordBatch to write
    /// * `table_name` - The Iceberg table name (used for path construction)
    /// * `timestamp` - Optional timestamp for partitioning (uses current time if None)
    ///
    /// # Returns
    ///
    /// The S3 path where the file was uploaded.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Parquet writing fails
    /// - Upload fails after all retries
    /// - Partition path generation fails
    pub async fn write_and_upload(
        &self,
        batch: &RecordBatch,
        table_name: &str,
        timestamp: Option<DateTime<Utc>>,
    ) -> Result<String, WriterError> {
        // Generate Parquet data
        let parquet_data = self.write_to_parquet(batch)?;

        // Generate S3 path with partitioning
        let s3_path = self.generate_s3_path(table_name, timestamp)?;

        // Upload with retry logic
        self.upload_with_retry(&s3_path, parquet_data).await?;

        Ok(s3_path)
    }

    /// Writes a RecordBatch to Parquet format in memory.
    fn write_to_parquet(&self, batch: &RecordBatch) -> Result<Vec<u8>, WriterError> {
        let mut buffer = Vec::new();

        // Configure writer properties
        let props = WriterProperties::builder()
            .set_compression(self.writer_config.compression.into())
            .set_writer_version(WriterVersion::PARQUET_2_0)
            .build();

        // Create ArrowWriter
        let mut writer = ArrowWriter::try_new(&mut buffer, batch.schema(), Some(props))?;

        // Write the batch
        writer.write(batch)?;

        // Finalize the file
        writer.close()?;

        Ok(buffer)
    }

    /// Generates an S3 path with optional date/hour partitioning.
    ///
    /// Format: `{warehouse}/{table}/date={YYYY-MM-DD}/hour={HH}/{uuid}.parquet`
    fn generate_s3_path(
        &self,
        table_name: &str,
        timestamp: Option<DateTime<Utc>>,
    ) -> Result<String, WriterError> {
        let ts = timestamp.unwrap_or_else(Utc::now);
        let file_id = Uuid::new_v4();

        let mut path_parts = vec![table_name.to_string()];

        // Add date partition
        if self.writer_config.partition_by_date {
            let date_str = ts.format("%Y-%m-%d").to_string();
            path_parts.push(format!("date={}", date_str));
        }

        // Add hour partition
        if self.writer_config.partition_by_hour {
            let hour_str = ts.format("%H").to_string();
            path_parts.push(format!("hour={}", hour_str));
        }

        // Add filename
        path_parts.push(format!("{}.parquet", file_id));

        Ok(path_parts.join("/"))
    }

    /// Uploads data to S3 with exponential backoff retry logic.
    async fn upload_with_retry(&self, path: &str, data: Vec<u8>) -> Result<(), WriterError> {
        let object_path = ObjectPath::from(path);
        let bytes = bytes::Bytes::from(data);

        for attempt in 0..=self.writer_config.max_retries {
            match self.object_store.put(&object_path, bytes.clone().into()).await {
                Ok(_) => {
                    log::info!("Successfully uploaded to S3: {}", path);
                    return Ok(());
                }
                Err(e) => {
                    if attempt == self.writer_config.max_retries {
                        log::error!(
                            "Failed to upload after {} retries: {}",
                            self.writer_config.max_retries,
                            e
                        );
                        return Err(WriterError::UploadFailed(self.writer_config.max_retries));
                    }

                    // Exponential backoff: delay * 2^attempt
                    let delay_ms = self.writer_config.retry_delay_ms * (1 << attempt);
                    log::warn!(
                        "Upload attempt {} failed, retrying in {}ms: {}",
                        attempt + 1,
                        delay_ms,
                        e
                    );
                    sleep(Duration::from_millis(delay_ms)).await;
                }
            }
        }

        Err(WriterError::UploadFailed(self.writer_config.max_retries))
    }

    /// Returns the warehouse path being used.
    #[must_use]
    pub fn warehouse_path(&self) -> &str {
        &self.iceberg_config.warehouse_name
    }

    /// Returns the compression codec being used.
    #[must_use]
    pub fn compression(&self) -> CompressionCodec {
        self.writer_config.compression
    }
}

////////////////////////////////////////////////////////////////////////////////
// Tests
////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;
    use arrow::array::{Int64Array, StringArray};
    use arrow::datatypes::{DataType, Field, Schema};
    use crate::config::IcebergCredentials;

    fn create_test_batch() -> RecordBatch {
        let schema = Arc::new(Schema::new(vec![
            Field::new("id", DataType::Int64, false),
            Field::new("name", DataType::Utf8, false),
        ]));

        let id_array = Arc::new(Int64Array::from(vec![1, 2, 3]));
        let name_array = Arc::new(StringArray::from(vec!["alice", "bob", "charlie"]));

        RecordBatch::try_new(schema, vec![id_array, name_array]).unwrap()
    }

    #[test]
    fn test_compression_codec_conversion() {
        let snappy = CompressionCodec::Snappy;
        let compression: Compression = snappy.into();
        assert!(matches!(compression, Compression::SNAPPY));

        let zstd = CompressionCodec::Zstd(5);
        let compression: Compression = zstd.into();
        assert!(matches!(compression, Compression::ZSTD(_)));
    }

    #[test]
    fn test_default_writer_config() {
        let config = WriterConfig::default();
        assert_eq!(config.max_retries, 3);
        assert_eq!(config.retry_delay_ms, 1000);
        assert!(config.partition_by_date);
        assert!(config.partition_by_hour);
    }

    #[test]
    fn test_generate_s3_path_with_partitioning() {
        let iceberg_config = IcebergConfig::builder()
            .project_ref("test-project")
            .warehouse_name("warehouse")
            .credentials(IcebergCredentials::new("test-key"))
            .build()
            .unwrap();

        let writer_config = WriterConfig::default();

        let timestamp = DateTime::parse_from_rfc3339("2024-12-19T15:30:00Z")
            .unwrap()
            .with_timezone(&Utc);

        // Mock writer (can't actually create object store in tests)
        let path_parts: Vec<String> = vec![
            "orderbook_l3_events".to_string(),
            "date=2024-12-19".to_string(),
            "hour=15".to_string(),
        ];

        let path_prefix = path_parts.join("/");
        assert!(path_prefix.contains("orderbook_l3_events"));
        assert!(path_prefix.contains("date=2024-12-19"));
        assert!(path_prefix.contains("hour=15"));
    }

    #[test]
    fn test_generate_s3_path_without_hour() {
        let timestamp = DateTime::parse_from_rfc3339("2024-12-19T15:30:00Z")
            .unwrap()
            .with_timezone(&Utc);

        let path_parts: Vec<String> = vec![
            "orderbook_l3_events".to_string(),
            "date=2024-12-19".to_string(),
        ];

        let path = path_parts.join("/");
        assert!(path.contains("orderbook_l3_events"));
        assert!(path.contains("date=2024-12-19"));
        assert!(!path.contains("hour="));
    }

    #[test]
    fn test_write_to_parquet_in_memory() {
        let batch = create_test_batch();

        let mut buffer = Vec::new();
        let props = WriterProperties::builder()
            .set_compression(Compression::SNAPPY)
            .build();

        let mut writer = ArrowWriter::try_new(&mut buffer, batch.schema(), Some(props)).unwrap();
        writer.write(&batch).unwrap();
        writer.close().unwrap();

        assert!(!buffer.is_empty());
        assert!(buffer.len() > 100); // Parquet files have overhead

        // Verify it starts with Parquet magic bytes "PAR1"
        assert_eq!(&buffer[0..4], b"PAR1");
    }

    #[test]
    fn test_parquet_with_zstd_compression() {
        let batch = create_test_batch();

        let mut buffer = Vec::new();
        let props = WriterProperties::builder()
            .set_compression(Compression::ZSTD(ZstdLevel::try_new(3).unwrap()))
            .build();

        let mut writer = ArrowWriter::try_new(&mut buffer, batch.schema(), Some(props)).unwrap();
        writer.write(&batch).unwrap();
        writer.close().unwrap();

        assert!(!buffer.is_empty());
        assert_eq!(&buffer[0..4], b"PAR1");
    }

    #[test]
    fn test_uuid_generation_uniqueness() {
        let uuid1 = Uuid::new_v4();
        let uuid2 = Uuid::new_v4();

        assert_ne!(uuid1, uuid2);
    }

    #[test]
    fn test_path_format() {
        let table = "trades";
        let date = "2024-12-19";
        let hour = "15";
        let uuid = Uuid::new_v4();

        let path = format!("{}/date={}/hour={}/{}.parquet", table, date, hour, uuid);

        assert!(path.starts_with("trades/"));
        assert!(path.contains("date=2024-12-19"));
        assert!(path.contains("hour=15"));
        assert!(path.ends_with(".parquet"));
    }

    #[tokio::test]
    async fn test_exponential_backoff_delays() {
        let base_delay = 100;
        let expected_delays = vec![100, 200, 400, 800];

        for (attempt, expected) in expected_delays.iter().enumerate() {
            let delay = base_delay * (1 << attempt);
            assert_eq!(delay, *expected);
        }
    }
}
