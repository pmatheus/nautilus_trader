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

//! Iceberg REST catalog client for Supabase integration.
//!
//! This module provides a REST client for the Supabase Iceberg catalog API,
//! implementing table metadata management, atomic commits, and snapshot tracking.

use std::collections::HashMap;

use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::config::IcebergConfig;

/// Errors that can occur during catalog operations.
#[derive(Error, Debug)]
pub enum CatalogError {
    /// HTTP request failed.
    #[error("HTTP request failed: {0}")]
    HttpError(#[from] reqwest::Error),

    /// Catalog API returned an error response.
    #[error("Catalog API error: {0}")]
    ApiError(String),

    /// Table not found in catalog.
    #[error("Table not found: {0}")]
    TableNotFound(String),

    /// Commit conflict (optimistic locking failure).
    #[error("Commit conflict: {0}")]
    CommitConflict(String),

    /// Serialization/deserialization error.
    #[error("Serialization error: {0}")]
    SerdeError(#[from] serde_json::Error),

    /// Invalid catalog response.
    #[error("Invalid catalog response: {0}")]
    InvalidResponse(String),

    /// Unauthorized access.
    #[error("Unauthorized: {0}")]
    Unauthorized(String),
}

/// Represents a table in the Iceberg catalog.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableMetadata {
    /// Current snapshot ID.
    #[serde(rename = "current-snapshot-id")]
    pub current_snapshot_id: Option<i64>,

    /// Metadata location in S3.
    #[serde(rename = "metadata-location")]
    pub metadata_location: String,

    /// Table format version.
    #[serde(rename = "format-version")]
    pub format_version: i32,

    /// Table UUID.
    pub uuid: String,

    /// Last updated timestamp.
    #[serde(rename = "last-updated-ms")]
    pub last_updated_ms: i64,
}

/// Request body for committing a new snapshot.
#[derive(Debug, Clone, Serialize)]
pub struct CommitRequest {
    /// Current snapshot ID for optimistic locking.
    #[serde(rename = "current-snapshot-id")]
    pub current_snapshot_id: Option<i64>,

    /// New manifest files to add.
    pub manifests: Vec<ManifestFile>,

    /// Summary of the commit.
    pub summary: HashMap<String, String>,
}

/// Represents a manifest file in Iceberg.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManifestFile {
    /// S3 path to the manifest file.
    pub path: String,

    /// Length of the manifest file in bytes.
    pub length: i64,

    /// Partition spec ID.
    #[serde(rename = "partition-spec-id")]
    pub partition_spec_id: i32,

    /// Number of added data files.
    #[serde(rename = "added-data-files-count")]
    pub added_data_files_count: i32,
}

/// Response from a commit operation.
#[derive(Debug, Clone, Deserialize)]
pub struct CommitResponse {
    /// Updated metadata location.
    #[serde(rename = "metadata-location")]
    pub metadata_location: String,

    /// New current snapshot ID.
    #[serde(rename = "current-snapshot-id")]
    pub current_snapshot_id: i64,
}

/// REST catalog client for Supabase Iceberg.
///
/// This client implements the Iceberg REST catalog API for managing table
/// metadata, committing new snapshots, and tracking manifest files.
///
/// # Example
///
/// ```no_run
/// use nautilus_iceberg::catalog::CatalogClient;
/// use nautilus_iceberg::config::{IcebergConfig, IcebergCredentials};
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let config = IcebergConfig::builder()
///     .project_ref("my-project")
///     .credentials(IcebergCredentials::new("api-key"))
///     .build()?;
///
/// let client = CatalogClient::new(config);
///
/// let metadata = client.get_table("default", "orderbook_l3_events").await?;
/// println!("Current snapshot: {:?}", metadata.current_snapshot_id);
/// # Ok(())
/// # }
/// ```
pub struct CatalogClient {
    config: IcebergConfig,
    client: Client,
    base_url: String,
}

impl CatalogClient {
    /// Creates a new catalog client.
    ///
    /// # Arguments
    ///
    /// * `config` - Iceberg configuration with catalog URI and credentials
    #[must_use]
    pub fn new(config: IcebergConfig) -> Self {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .expect("Failed to create HTTP client");

        let base_url = config.catalog_uri();

        Self {
            config,
            client,
            base_url,
        }
    }

    /// Gets metadata for a table from the catalog.
    ///
    /// # Arguments
    ///
    /// * `namespace` - The namespace (e.g., "default")
    /// * `table_name` - The table name (e.g., "orderbook_l3_events")
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The HTTP request fails
    /// - The table is not found
    /// - The response cannot be parsed
    pub async fn get_table(
        &self,
        namespace: &str,
        table_name: &str,
    ) -> Result<TableMetadata, CatalogError> {
        let url = format!(
            "{}/v1/{}/namespaces/{}/tables/{}",
            self.base_url, self.config.warehouse_name, namespace, table_name
        );

        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.config.credentials.api_key))
            .send()
            .await?;

        match response.status() {
            StatusCode::OK => {
                let metadata: TableMetadata = response.json().await?;
                Ok(metadata)
            }
            StatusCode::NOT_FOUND => {
                Err(CatalogError::TableNotFound(format!("{}.{}", namespace, table_name)))
            }
            StatusCode::UNAUTHORIZED => {
                Err(CatalogError::Unauthorized("Invalid API key".to_string()))
            }
            status => {
                let error_text = response.text().await.unwrap_or_default();
                Err(CatalogError::ApiError(format!(
                    "Status {}: {}",
                    status, error_text
                )))
            }
        }
    }

    /// Creates a new table in the catalog.
    ///
    /// # Arguments
    ///
    /// * `namespace` - The namespace
    /// * `table_name` - The table name
    /// * `schema` - The Arrow schema definition (as JSON)
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The HTTP request fails
    /// - The table already exists
    /// - The schema is invalid
    pub async fn create_table(
        &self,
        namespace: &str,
        table_name: &str,
        schema: serde_json::Value,
    ) -> Result<TableMetadata, CatalogError> {
        let url = format!(
            "{}/v1/{}/namespaces/{}/tables",
            self.base_url, self.config.warehouse_name, namespace
        );

        let request_body = serde_json::json!({
            "name": table_name,
            "schema": schema,
            "partition-spec": [],
            "write-order": [],
        });

        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.config.credentials.api_key))
            .json(&request_body)
            .send()
            .await?;

        match response.status() {
            StatusCode::OK | StatusCode::CREATED => {
                let metadata: TableMetadata = response.json().await?;
                Ok(metadata)
            }
            StatusCode::CONFLICT => {
                Err(CatalogError::ApiError(format!("Table {}.{} already exists", namespace, table_name)))
            }
            StatusCode::UNAUTHORIZED => {
                Err(CatalogError::Unauthorized("Invalid API key".to_string()))
            }
            status => {
                let error_text = response.text().await.unwrap_or_default();
                Err(CatalogError::ApiError(format!(
                    "Status {}: {}",
                    status, error_text
                )))
            }
        }
    }

    /// Commits a new snapshot to the table using atomic compare-and-swap.
    ///
    /// # Arguments
    ///
    /// * `namespace` - The namespace
    /// * `table_name` - The table name
    /// * `commit` - The commit request with manifest files
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The HTTP request fails
    /// - There is a commit conflict (concurrent modification)
    /// - The manifest files are invalid
    ///
    /// # Note
    ///
    /// This operation uses optimistic locking. If another client commits a snapshot
    /// between reading and committing, this will fail with `CommitConflict`.
    pub async fn commit_snapshot(
        &self,
        namespace: &str,
        table_name: &str,
        commit: CommitRequest,
    ) -> Result<CommitResponse, CatalogError> {
        let url = format!(
            "{}/v1/{}/namespaces/{}/tables/{}/commits",
            self.base_url, self.config.warehouse_name, namespace, table_name
        );

        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.config.credentials.api_key))
            .json(&commit)
            .send()
            .await?;

        match response.status() {
            StatusCode::OK => {
                let commit_response: CommitResponse = response.json().await?;
                Ok(commit_response)
            }
            StatusCode::CONFLICT => {
                Err(CatalogError::CommitConflict(
                    "Concurrent modification detected, retry with latest snapshot".to_string(),
                ))
            }
            StatusCode::NOT_FOUND => {
                Err(CatalogError::TableNotFound(format!("{}.{}", namespace, table_name)))
            }
            StatusCode::UNAUTHORIZED => {
                Err(CatalogError::Unauthorized("Invalid API key".to_string()))
            }
            status => {
                let error_text = response.text().await.unwrap_or_default();
                Err(CatalogError::ApiError(format!(
                    "Status {}: {}",
                    status, error_text
                )))
            }
        }
    }

    /// Returns the catalog URI being used.
    #[must_use]
    pub fn catalog_uri(&self) -> &str {
        &self.base_url
    }

    /// Returns the warehouse name.
    #[must_use]
    pub fn warehouse_name(&self) -> &str {
        &self.config.warehouse_name
    }
}

////////////////////////////////////////////////////////////////////////////////
// Tests
////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::IcebergCredentials;

    fn create_test_config() -> IcebergConfig {
        IcebergConfig::builder()
            .project_ref("test-project")
            .warehouse_name("test-warehouse")
            .credentials(IcebergCredentials::new("test-api-key"))
            .build()
            .unwrap()
    }

    #[test]
    fn test_catalog_client_creation() {
        let config = create_test_config();
        let client = CatalogClient::new(config);

        assert_eq!(
            client.catalog_uri(),
            "https://test-project.supabase.co/storage/v1/iceberg"
        );
        assert_eq!(client.warehouse_name(), "test-warehouse");
    }

    #[test]
    fn test_table_metadata_serialization() {
        let metadata = TableMetadata {
            current_snapshot_id: Some(12345),
            metadata_location: "s3://warehouse/metadata/v1.json".to_string(),
            format_version: 2,
            uuid: "uuid-123".to_string(),
            last_updated_ms: 1234567890,
        };

        let json = serde_json::to_string(&metadata).unwrap();
        let deserialized: TableMetadata = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.current_snapshot_id, Some(12345));
        assert_eq!(deserialized.format_version, 2);
    }

    #[test]
    fn test_commit_request_serialization() {
        let mut summary = HashMap::new();
        summary.insert("operation".to_string(), "append".to_string());

        let commit = CommitRequest {
            current_snapshot_id: Some(100),
            manifests: vec![ManifestFile {
                path: "s3://warehouse/manifests/m1.avro".to_string(),
                length: 1024,
                partition_spec_id: 0,
                added_data_files_count: 10,
            }],
            summary,
        };

        let json = serde_json::to_string(&commit).unwrap();
        assert!(json.contains("current-snapshot-id"));
        assert!(json.contains("manifests"));
        assert!(json.contains("summary"));
    }

    #[test]
    fn test_manifest_file_creation() {
        let manifest = ManifestFile {
            path: "s3://bucket/manifest.avro".to_string(),
            length: 2048,
            partition_spec_id: 1,
            added_data_files_count: 5,
        };

        assert_eq!(manifest.length, 2048);
        assert_eq!(manifest.added_data_files_count, 5);
    }
}
