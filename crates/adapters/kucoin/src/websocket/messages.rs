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

//! Kucoin WebSocket message types.

use serde::{Deserialize, Serialize};

/// WebSocket subscription request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KucoinWsSubscription {
    /// Message ID
    pub id: String,
    /// Message type ("subscribe" or "unsubscribe")
    #[serde(rename = "type")]
    pub msg_type: String,
    /// Topic to subscribe to
    pub topic: String,
    /// Whether to use tunnel (private channels)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_channel: Option<bool>,
    /// Whether the response is required
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response: Option<bool>,
}

/// WebSocket welcome message from Kucoin.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KucoinWsWelcome {
    /// Message ID
    pub id: String,
    /// Message type ("welcome")
    #[serde(rename = "type")]
    pub msg_type: String,
}
