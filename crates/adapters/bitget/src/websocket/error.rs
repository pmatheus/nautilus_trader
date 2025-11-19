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

//! Error types for Bitget WebSocket client.

use thiserror::Error;

/// Errors that can occur when using the Bitget WebSocket client.
#[derive(Error, Debug, Clone)]
pub enum BitgetWsError {
    /// WebSocket connection failed.
    #[error("WebSocket connection failed: {0}")]
    ConnectionFailed(String),

    /// Failed to parse message.
    #[error("Failed to parse message: {0}")]
    ParseError(String),

    /// Subscription failed.
    #[error("Subscription failed: {0}")]
    SubscriptionFailed(String),

    /// Authentication failed.
    #[error("Authentication failed: {0}")]
    AuthenticationError(String),

    /// Connection closed unexpectedly.
    #[error("Connection closed: {0}")]
    ConnectionClosed(String),
}
