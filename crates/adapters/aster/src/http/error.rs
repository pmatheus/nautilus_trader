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

use serde::{Deserialize, Serialize};
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

/// Aster API error response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AsterErrorResponse {
    pub code: i32,
    pub msg: String,
}

/// Comprehensive error type for Aster adapter.
#[derive(Debug, Error)]
pub enum Error {
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),

    #[error("WebSocket error: {0}")]
    WebSocket(String),

    #[error("API error (code {code}): {message}")]
    Api { code: i32, message: String },

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Transport error: {0}")]
    Transport(String),

    #[error("Parse error: {0}")]
    Parse(String),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Rate limit exceeded")]
    RateLimitExceeded,

    #[error("Timeout")]
    Timeout,

    #[error("Invalid signature")]
    InvalidSignature,

    #[error("Authentication required")]
    AuthenticationRequired,

    #[error("Unknown error: {0}")]
    Unknown(String),
}

impl Error {
    pub fn api(code: i32, message: impl Into<String>) -> Self {
        Self::Api {
            code,
            message: message.into(),
        }
    }

    pub fn bad_request(message: impl Into<String>) -> Self {
        Self::BadRequest(message.into())
    }

    pub fn transport(message: impl Into<String>) -> Self {
        Self::Transport(message.into())
    }

    pub fn parse(message: impl Into<String>) -> Self {
        Self::Parse(message.into())
    }

    pub fn websocket(message: impl Into<String>) -> Self {
        Self::WebSocket(message.into())
    }

    pub fn unknown(message: impl Into<String>) -> Self {
        Self::Unknown(message.into())
    }

    /// Returns true if this is a rate limit error.
    pub fn is_rate_limit(&self) -> bool {
        matches!(self, Self::RateLimitExceeded)
            || matches!(self, Self::Api { code: 429, .. })
    }

    /// Returns true if this is a retryable error.
    pub fn is_retryable(&self) -> bool {
        match self {
            Self::Http(_) | Self::Timeout | Self::RateLimitExceeded => true,
            Self::Api { code, .. } => matches!(code, 429 | 500 | 502 | 503 | 504),
            _ => false,
        }
    }
}
