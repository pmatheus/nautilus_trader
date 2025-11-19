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

//! Error types for Bitget HTTP client.

use thiserror::Error;

/// Errors that can occur when using the Bitget HTTP client.
#[derive(Error, Debug, Clone)]
pub enum BitgetHttpError {
    /// HTTP request failed.
    #[error("HTTP request failed: {0}")]
    RequestFailed(String),

    /// Failed to parse response.
    #[error("Failed to parse response: {0}")]
    ParseError(String),

    /// API error response.
    #[error("API error {code}: {msg}")]
    ApiError {
        /// Error code from Bitget API.
        code: String,
        /// Error message from Bitget API.
        msg: String,
    },

    /// Rate limit exceeded.
    #[error("Rate limit exceeded")]
    RateLimitExceeded,

    /// Authentication error.
    #[error("Authentication failed: {0}")]
    AuthenticationError(String),

    /// Invalid request parameters.
    #[error("Invalid request parameters: {0}")]
    InvalidParameters(String),
}
