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

//! Error types for Bitget WebSocket operations.

use thiserror::Error;

/// Errors that can occur when interacting with the Bitget WebSocket API.
#[derive(Error, Debug, Clone)]
pub enum BitgetWsError {
    /// WebSocket connection failed.
    #[error("WebSocket connection failed: {0}")]
    Connection(String),

    /// Failed to send message.
    #[error("Failed to send message: {0}")]
    Send(String),

    /// Failed to receive message.
    #[error("Failed to receive message: {0}")]
    Receive(String),

    /// Failed to deserialize message.
    #[error("Failed to deserialize message: {0}")]
    Deserialization(String),

    /// Failed to serialize message.
    #[error("Failed to serialize message: {0}")]
    Serialization(String),

    /// Authentication failed.
    #[error("Authentication failed: {0}")]
    Authentication(String),

    /// Subscription failed.
    #[error("Subscription failed: {0}")]
    Subscription(String),

    /// API error response.
    #[error("API error (code: {code}): {msg}")]
    ApiError {
        /// Error code
        code: String,
        /// Error message
        msg: String,
    },

    /// Connection timeout.
    #[error("Connection timeout")]
    Timeout,

    /// Connection closed.
    #[error("Connection closed: {0}")]
    Closed(String),

    /// Invalid message format.
    #[error("Invalid message format: {0}")]
    InvalidMessage(String),

    /// Operation cancelled.
    #[error("Operation cancelled")]
    Cancelled,
}

/// Result type for Bitget WebSocket operations.
pub type BitgetWsResult<T> = Result<T, BitgetWsError>;
