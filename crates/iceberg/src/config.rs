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

//! Configuration types for Apache Iceberg integration.

use serde::{Deserialize, Serialize};

/// Configuration for Iceberg catalog connection.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IcebergCatalogConfig {
    /// The catalog type (e.g., "rest", "hive", "glue").
    pub catalog_type: String,
    /// The catalog URI or endpoint.
    pub uri: String,
    /// Warehouse location for storing table data.
    pub warehouse: String,
}

impl Default for IcebergCatalogConfig {
    fn default() -> Self {
        Self {
            catalog_type: "rest".to_string(),
            uri: "http://localhost:8181".to_string(),
            warehouse: "s3://warehouse".to_string(),
        }
    }
}
