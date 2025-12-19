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

//! Error types for Bitget HTTP operations.

use thiserror::Error;

/// Errors that can occur when interacting with the Bitget HTTP API.
#[derive(Error, Debug, Clone)]
pub enum BitgetHttpError {
    /// Failed to build HTTP client.
    #[error("Failed to build HTTP client: {0}")]
    ClientBuild(String),

    /// HTTP request failed.
    #[error("HTTP request failed: {0}")]
    Request(String),

    /// Failed to deserialize response.
    #[error("Failed to deserialize response: {0}")]
    Deserialization(String),

    /// API returned an error response.
    #[error("API error (code: {code}): {msg}")]
    ApiError {
        /// Bitget error code
        code: String,
        /// Bitget error message
        msg: String,
    },

    /// Rate limit exceeded.
    #[error("Rate limit exceeded: {0}")]
    RateLimit(String),

    /// Authentication failed.
    #[error("Authentication failed: {0}")]
    Authentication(String),

    /// Invalid request parameters.
    #[error("Invalid request parameters: {0}")]
    InvalidParameters(String),

    /// Response timeout.
    #[error("Request timeout after {0}ms")]
    Timeout(u64),

    /// Operation cancelled.
    #[error("Operation cancelled")]
    Cancelled,
}

impl BitgetHttpError {
    /// Returns `true` if this error is retryable.
    #[must_use]
    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            Self::Request(_) | Self::RateLimit(_) | Self::Timeout(_)
        )
    }

    /// Returns `true` if this is a rate limit error.
    #[must_use]
    pub fn is_rate_limit(&self) -> bool {
        matches!(self, Self::RateLimit(_))
    }
}

/// Result type for Bitget HTTP operations.
pub type BitgetHttpResult<T> = Result<T, BitgetHttpError>;
