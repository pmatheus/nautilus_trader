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

//! Configuration types for Apache Iceberg integration with Supabase.

use std::env;

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Errors that can occur during configuration validation.
#[derive(Error, Debug)]
pub enum ConfigError {
    /// Missing required configuration field.
    #[error("Missing required field: {0}")]
    MissingField(&'static str),

    /// Invalid URL format.
    #[error("Invalid URL format for {0}: {1}")]
    InvalidUrl(&'static str, String),

    /// Missing credentials.
    #[error("Missing credentials: {0}")]
    MissingCredentials(&'static str),

    /// Invalid configuration value.
    #[error("Invalid configuration value for {0}: {1}")]
    InvalidValue(&'static str, String),
}

/// Authentication credentials for Supabase Iceberg access.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IcebergCredentials {
    /// Supabase API key or service role key.
    pub api_key: String,
}

impl IcebergCredentials {
    /// Creates new credentials with the provided API key.
    #[must_use]
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            api_key: api_key.into(),
        }
    }

    /// Creates credentials from environment variables.
    ///
    /// Reads from `SUPABASE_API_KEY` environment variable.
    ///
    /// # Errors
    ///
    /// Returns an error if the environment variable is not set.
    pub fn from_env() -> Result<Self, ConfigError> {
        let api_key = env::var("SUPABASE_API_KEY")
            .map_err(|_| ConfigError::MissingCredentials("SUPABASE_API_KEY not set"))?;
        Ok(Self { api_key })
    }
}

/// Configuration for connecting to Supabase Iceberg catalog.
///
/// This configuration manages connection details for writing L3 orderbook data
/// to Apache Iceberg tables hosted on Supabase.
///
/// # Example
///
/// ```no_run
/// use nautilus_iceberg::config::{IcebergConfig, IcebergCredentials};
///
/// let config = IcebergConfig::builder()
///     .project_ref("abc123xyz")
///     .credentials(IcebergCredentials::new("your-api-key"))
///     .batch_size(5000)
///     .build()
///     .unwrap();
///
/// assert_eq!(config.s3_endpoint(), "https://abc123xyz.supabase.co/storage/v1/s3");
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IcebergConfig {
    /// Supabase project reference ID.
    pub project_ref: String,
    /// Warehouse name (default: "warehouse").
    pub warehouse_name: String,
    /// Authentication credentials.
    pub credentials: IcebergCredentials,
    /// Batch size for buffering events before writing (default: 10000).
    pub batch_size: usize,
    /// Flush interval in seconds (default: 60).
    pub flush_interval_secs: u64,
}

impl IcebergConfig {
    /// Creates a new builder for constructing an `IcebergConfig`.
    #[must_use]
    pub fn builder() -> IcebergConfigBuilder {
        IcebergConfigBuilder::default()
    }

    /// Returns the S3 endpoint URL for Supabase storage.
    ///
    /// Format: `https://{project_ref}.supabase.co/storage/v1/s3`
    #[must_use]
    pub fn s3_endpoint(&self) -> String {
        format!(
            "https://{}.supabase.co/storage/v1/s3",
            self.project_ref
        )
    }

    /// Returns the catalog URI for the Iceberg REST catalog.
    ///
    /// Format: `https://{project_ref}.supabase.co/storage/v1/iceberg`
    #[must_use]
    pub fn catalog_uri(&self) -> String {
        format!(
            "https://{}.supabase.co/storage/v1/iceberg",
            self.project_ref
        )
    }

    /// Returns the warehouse path.
    ///
    /// Format: `s3://{warehouse_name}`
    #[must_use]
    pub fn warehouse_path(&self) -> String {
        format!("s3://{}", self.warehouse_name)
    }

    /// Validates the configuration.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The project reference is empty
    /// - The warehouse name is empty
    /// - The batch size is zero
    /// - The API key is empty
    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.project_ref.is_empty() {
            return Err(ConfigError::MissingField("project_ref"));
        }

        if self.warehouse_name.is_empty() {
            return Err(ConfigError::MissingField("warehouse_name"));
        }

        if self.batch_size == 0 {
            return Err(ConfigError::InvalidValue(
                "batch_size",
                "must be greater than 0".to_string(),
            ));
        }

        if self.credentials.api_key.is_empty() {
            return Err(ConfigError::MissingCredentials("api_key cannot be empty"));
        }

        // Validate project_ref format (alphanumeric and hyphens)
        if !self
            .project_ref
            .chars()
            .all(|c| c.is_alphanumeric() || c == '-')
        {
            return Err(ConfigError::InvalidValue(
                "project_ref",
                "must contain only alphanumeric characters and hyphens".to_string(),
            ));
        }

        Ok(())
    }
}

/// Builder for constructing an `IcebergConfig`.
#[derive(Debug, Default)]
pub struct IcebergConfigBuilder {
    project_ref: Option<String>,
    warehouse_name: Option<String>,
    credentials: Option<IcebergCredentials>,
    batch_size: Option<usize>,
    flush_interval_secs: Option<u64>,
}

impl IcebergConfigBuilder {
    /// Sets the Supabase project reference.
    #[must_use]
    pub fn project_ref(mut self, project_ref: impl Into<String>) -> Self {
        self.project_ref = Some(project_ref.into());
        self
    }

    /// Sets the warehouse name (default: "warehouse").
    #[must_use]
    pub fn warehouse_name(mut self, warehouse_name: impl Into<String>) -> Self {
        self.warehouse_name = Some(warehouse_name.into());
        self
    }

    /// Sets the authentication credentials.
    #[must_use]
    pub fn credentials(mut self, credentials: IcebergCredentials) -> Self {
        self.credentials = Some(credentials);
        self
    }

    /// Sets the batch size for buffering events (default: 10000).
    #[must_use]
    pub fn batch_size(mut self, batch_size: usize) -> Self {
        self.batch_size = Some(batch_size);
        self
    }

    /// Sets the flush interval in seconds (default: 60).
    #[must_use]
    pub fn flush_interval_secs(mut self, flush_interval_secs: u64) -> Self {
        self.flush_interval_secs = Some(flush_interval_secs);
        self
    }

    /// Builds the `IcebergConfig`, validating all required fields.
    ///
    /// # Errors
    ///
    /// Returns an error if required fields are missing or invalid.
    pub fn build(self) -> Result<IcebergConfig, ConfigError> {
        let config = IcebergConfig {
            project_ref: self
                .project_ref
                .ok_or(ConfigError::MissingField("project_ref"))?,
            warehouse_name: self.warehouse_name.unwrap_or_else(|| "warehouse".to_string()),
            credentials: self
                .credentials
                .ok_or(ConfigError::MissingField("credentials"))?,
            batch_size: self.batch_size.unwrap_or(10_000),
            flush_interval_secs: self.flush_interval_secs.unwrap_or(60),
        };

        config.validate()?;
        Ok(config)
    }
}

/// Legacy configuration for Iceberg catalog connection.
///
/// This is maintained for backward compatibility with the original placeholder.
/// New code should use `IcebergConfig` instead.
#[deprecated(since = "0.1.0", note = "Use IcebergConfig instead")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IcebergCatalogConfig {
    /// The catalog type (e.g., "rest", "hive", "glue").
    pub catalog_type: String,
    /// The catalog URI or endpoint.
    pub uri: String,
    /// Warehouse location for storing table data.
    pub warehouse: String,
}

#[allow(deprecated)]
impl Default for IcebergCatalogConfig {
    fn default() -> Self {
        Self {
            catalog_type: "rest".to_string(),
            uri: "http://localhost:8181".to_string(),
            warehouse: "s3://warehouse".to_string(),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// Tests
////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_credentials_new() {
        let creds = IcebergCredentials::new("test-key");
        assert_eq!(creds.api_key, "test-key");
    }

    #[test]
    fn test_config_builder_minimal() {
        let config = IcebergConfig::builder()
            .project_ref("abc123xyz")
            .credentials(IcebergCredentials::new("test-key"))
            .build()
            .unwrap();

        assert_eq!(config.project_ref, "abc123xyz");
        assert_eq!(config.warehouse_name, "warehouse");
        assert_eq!(config.batch_size, 10_000);
        assert_eq!(config.flush_interval_secs, 60);
    }

    #[test]
    fn test_config_builder_full() {
        let config = IcebergConfig::builder()
            .project_ref("xyz789abc")
            .warehouse_name("my-warehouse")
            .credentials(IcebergCredentials::new("api-key-123"))
            .batch_size(5000)
            .flush_interval_secs(30)
            .build()
            .unwrap();

        assert_eq!(config.project_ref, "xyz789abc");
        assert_eq!(config.warehouse_name, "my-warehouse");
        assert_eq!(config.batch_size, 5000);
        assert_eq!(config.flush_interval_secs, 30);
        assert_eq!(config.credentials.api_key, "api-key-123");
    }

    #[test]
    fn test_s3_endpoint_url() {
        let config = IcebergConfig::builder()
            .project_ref("testproject123")
            .credentials(IcebergCredentials::new("key"))
            .build()
            .unwrap();

        assert_eq!(
            config.s3_endpoint(),
            "https://testproject123.supabase.co/storage/v1/s3"
        );
    }

    #[test]
    fn test_catalog_uri() {
        let config = IcebergConfig::builder()
            .project_ref("testproject123")
            .credentials(IcebergCredentials::new("key"))
            .build()
            .unwrap();

        assert_eq!(
            config.catalog_uri(),
            "https://testproject123.supabase.co/storage/v1/iceberg"
        );
    }

    #[test]
    fn test_warehouse_path() {
        let config = IcebergConfig::builder()
            .project_ref("test")
            .warehouse_name("my-warehouse")
            .credentials(IcebergCredentials::new("key"))
            .build()
            .unwrap();

        assert_eq!(config.warehouse_path(), "s3://my-warehouse");
    }

    #[test]
    fn test_builder_missing_project_ref() {
        let result = IcebergConfig::builder()
            .credentials(IcebergCredentials::new("key"))
            .build();

        assert!(matches!(
            result,
            Err(ConfigError::MissingField("project_ref"))
        ));
    }

    #[test]
    fn test_builder_missing_credentials() {
        let result = IcebergConfig::builder().project_ref("test").build();

        assert!(matches!(
            result,
            Err(ConfigError::MissingField("credentials"))
        ));
    }

    #[test]
    fn test_validation_empty_project_ref() {
        let config = IcebergConfig {
            project_ref: String::new(),
            warehouse_name: "warehouse".to_string(),
            credentials: IcebergCredentials::new("key"),
            batch_size: 1000,
            flush_interval_secs: 60,
        };

        assert!(matches!(
            config.validate(),
            Err(ConfigError::MissingField("project_ref"))
        ));
    }

    #[test]
    fn test_validation_zero_batch_size() {
        let config = IcebergConfig {
            project_ref: "test".to_string(),
            warehouse_name: "warehouse".to_string(),
            credentials: IcebergCredentials::new("key"),
            batch_size: 0,
            flush_interval_secs: 60,
        };

        assert!(matches!(
            config.validate(),
            Err(ConfigError::InvalidValue("batch_size", _))
        ));
    }

    #[test]
    fn test_validation_invalid_project_ref() {
        let config = IcebergConfig {
            project_ref: "test@project!".to_string(),
            warehouse_name: "warehouse".to_string(),
            credentials: IcebergCredentials::new("key"),
            batch_size: 1000,
            flush_interval_secs: 60,
        };

        assert!(matches!(
            config.validate(),
            Err(ConfigError::InvalidValue("project_ref", _))
        ));
    }

    #[test]
    fn test_validation_empty_api_key() {
        let config = IcebergConfig {
            project_ref: "test".to_string(),
            warehouse_name: "warehouse".to_string(),
            credentials: IcebergCredentials::new(""),
            batch_size: 1000,
            flush_interval_secs: 60,
        };

        assert!(matches!(
            config.validate(),
            Err(ConfigError::MissingCredentials(_))
        ));
    }

    #[test]
    fn test_config_serialization() {
        let config = IcebergConfig::builder()
            .project_ref("test123")
            .credentials(IcebergCredentials::new("key"))
            .build()
            .unwrap();

        let json = serde_json::to_string(&config).unwrap();
        let deserialized: IcebergConfig = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.project_ref, config.project_ref);
        assert_eq!(deserialized.warehouse_name, config.warehouse_name);
        assert_eq!(deserialized.batch_size, config.batch_size);
    }
}
