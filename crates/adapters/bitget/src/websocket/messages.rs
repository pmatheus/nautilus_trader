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

//! WebSocket message types for Bitget public and private channels.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use ustr::Ustr;

use super::enums::{BitgetInstType, BitgetWsOperation};

/// Bitget WebSocket subscription request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BitgetSubscription {
    /// Operation type
    pub op: BitgetWsOperation,
    /// Subscription arguments
    pub args: Vec<BitgetSubscriptionArg>,
}

/// Subscription argument specifying channel and instrument.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BitgetSubscriptionArg {
    /// Instrument type (e.g., "SPOT", "USDT-FUTURES")
    pub inst_type: BitgetInstType,
    /// Channel name (e.g., "trade", "books")
    pub channel: String,
    /// Instrument ID (e.g., "BTCUSDT")
    pub inst_id: Ustr,
}

/// Bitget WebSocket authentication request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BitgetAuthRequest {
    /// Operation type (always "login")
    pub op: BitgetWsOperation,
    /// Authentication arguments
    pub args: Vec<BitgetAuthArg>,
}

/// Authentication argument containing credentials.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BitgetAuthArg {
    /// API key
    pub api_key: String,
    /// Passphrase
    pub passphrase: String,
    /// Timestamp in milliseconds
    pub timestamp: String,
    /// Signature
    pub sign: String,
}

/// Bitget WebSocket ping message.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BitgetPing {
    /// Operation type (always "ping")
    pub op: BitgetWsOperation,
}

/// High-level message emitted by the Bitget WebSocket client.
#[derive(Debug, Clone)]
pub enum BitgetWsMessage {
    /// Generic response (subscribe/auth acknowledgement)
    Response(BitgetWsResponse),
    /// Authentication acknowledgement
    Auth(BitgetWsAuthResponse),
    /// Subscription acknowledgement
    Subscription(BitgetWsSubscriptionMsg),
    /// Order book snapshot or delta
    Orderbook(BitgetWsOrderbookMsg),
    /// Trade updates
    Trade(BitgetWsTradeMsg),
    /// Ticker updates
    Ticker(BitgetWsTickerMsg),
    /// Candlestick updates
    Candle(BitgetWsCandleMsg),
    /// Order updates from private channel
    Order(BitgetWsOrderMsg),
    /// Position updates from private channel
    Position(BitgetWsPositionMsg),
    /// Account updates from private channel
    Account(BitgetWsAccountMsg),
    /// Error received from the venue
    Error(BitgetWebSocketError),
    /// Pong response
    Pong,
    /// Raw message payload
    Raw(Value),
    /// Notification that the underlying connection reconnected
    Reconnected,
}

/// Standard Bitget WebSocket response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BitgetWsResponse {
    /// Event type
    pub event: Option<String>,
    /// Response code
    pub code: Option<String>,
    /// Response message
    pub msg: Option<String>,
    /// Request operation
    pub op: Option<String>,
}

/// Authentication response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BitgetWsAuthResponse {
    /// Event type (should be "login")
    pub event: String,
    /// Response code ("0" indicates success)
    pub code: String,
    /// Response message
    pub msg: String,
}

/// Subscription confirmation message.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BitgetWsSubscriptionMsg {
    /// Event type (should be "subscribe")
    pub event: String,
    /// Subscription argument
    pub arg: BitgetSubscriptionArg,
}

/// Order book message.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BitgetWsOrderbookMsg {
    /// Action type ("snapshot" or "update")
    pub action: String,
    /// Subscription argument
    pub arg: BitgetSubscriptionArg,
    /// Order book data
    pub data: Vec<BitgetWsOrderbookData>,
    /// Timestamp
    pub ts: String,
}

/// Order book data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BitgetWsOrderbookData {
    /// Asks (sell orders): [[price, quantity], ...]
    pub asks: Vec<[String; 2]>,
    /// Bids (buy orders): [[price, quantity], ...]
    pub bids: Vec<[String; 2]>,
    /// Checksum for validation
    #[serde(default)]
    pub checksum: i64,
    /// Timestamp
    pub ts: String,
}

/// Trade message.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BitgetWsTradeMsg {
    /// Action type
    pub action: String,
    /// Subscription argument
    pub arg: BitgetSubscriptionArg,
    /// Trade data
    pub data: Vec<BitgetWsTradeData>,
    /// Timestamp
    pub ts: String,
}

/// Trade data.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BitgetWsTradeData {
    /// Trade ID
    pub trade_id: String,
    /// Trade price
    pub price: String,
    /// Trade quantity
    pub size: String,
    /// Trade side ("buy" or "sell")
    pub side: String,
    /// Trade timestamp
    pub ts: String,
}

/// Ticker message.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BitgetWsTickerMsg {
    /// Action type
    pub action: String,
    /// Subscription argument
    pub arg: BitgetSubscriptionArg,
    /// Ticker data
    pub data: Vec<BitgetWsTickerData>,
    /// Timestamp
    pub ts: String,
}

/// Ticker data.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BitgetWsTickerData {
    /// Instrument ID
    pub inst_id: Ustr,
    /// Last price
    pub last_pr: String,
    /// Best bid price
    #[serde(default)]
    pub best_bid: String,
    /// Best ask price
    #[serde(default)]
    pub best_ask: String,
    /// 24h high
    #[serde(default)]
    pub high24h: String,
    /// 24h low
    #[serde(default)]
    pub low24h: String,
    /// 24h volume
    #[serde(default)]
    pub base_volume: String,
    /// 24h quote volume
    #[serde(default)]
    pub quote_volume: String,
    /// Timestamp
    pub ts: String,
}

/// Candlestick message.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BitgetWsCandleMsg {
    /// Action type
    pub action: String,
    /// Subscription argument
    pub arg: BitgetSubscriptionArg,
    /// Candle data
    pub data: Vec<BitgetWsCandleData>,
    /// Timestamp
    pub ts: String,
}

/// Candlestick data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BitgetWsCandleData {
    /// Candle data: [timestamp, open, high, low, close, volume, quote_volume]
    pub candle: Vec<String>,
}

/// Order update message (private channel).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BitgetWsOrderMsg {
    /// Action type
    pub action: String,
    /// Subscription argument
    pub arg: BitgetSubscriptionArg,
    /// Order data
    pub data: Vec<Value>,
    /// Timestamp
    pub ts: String,
}

/// Position update message (private channel).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BitgetWsPositionMsg {
    /// Action type
    pub action: String,
    /// Subscription argument
    pub arg: BitgetSubscriptionArg,
    /// Position data
    pub data: Vec<Value>,
    /// Timestamp
    pub ts: String,
}

/// Account update message (private channel).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BitgetWsAccountMsg {
    /// Action type
    pub action: String,
    /// Subscription argument
    pub arg: BitgetSubscriptionArg,
    /// Account data
    pub data: Vec<Value>,
    /// Timestamp
    pub ts: String,
}

/// WebSocket error message.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BitgetWebSocketError {
    /// Event type
    pub event: String,
    /// Error code
    pub code: String,
    /// Error message
    pub msg: String,
}
