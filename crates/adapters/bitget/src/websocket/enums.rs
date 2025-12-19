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

//! Enums for Bitget WebSocket operations and channels.

use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

/// WebSocket operation types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Display, EnumString)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum BitgetWsOperation {
    /// Subscribe to channels
    Subscribe,
    /// Unsubscribe from channels
    Unsubscribe,
    /// Login/authentication
    Login,
    /// Ping message
    Ping,
    /// Pong response
    Pong,
}

/// Public market data channels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Display, EnumString)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum BitgetWsPublicChannel {
    /// Order book updates
    #[serde(rename = "books")]
    #[strum(serialize = "books")]
    Books,
    /// Order book updates (books5 - top 5 levels)
    #[serde(rename = "books5")]
    #[strum(serialize = "books5")]
    Books5,
    /// Order book updates (books15 - top 15 levels)
    #[serde(rename = "books15")]
    #[strum(serialize = "books15")]
    Books15,
    /// Trade updates
    #[serde(rename = "trade")]
    #[strum(serialize = "trade")]
    Trade,
    /// Ticker updates
    #[serde(rename = "ticker")]
    #[strum(serialize = "ticker")]
    Ticker,
    /// Candlestick updates (1m)
    #[serde(rename = "candle1m")]
    #[strum(serialize = "candle1m")]
    Candle1m,
    /// Candlestick updates (5m)
    #[serde(rename = "candle5m")]
    #[strum(serialize = "candle5m")]
    Candle5m,
    /// Candlestick updates (15m)
    #[serde(rename = "candle15m")]
    #[strum(serialize = "candle15m")]
    Candle15m,
    /// Candlestick updates (30m)
    #[serde(rename = "candle30m")]
    #[strum(serialize = "candle30m")]
    Candle30m,
    /// Candlestick updates (1h)
    #[serde(rename = "candle1H")]
    #[strum(serialize = "candle1H")]
    Candle1h,
    /// Candlestick updates (4h)
    #[serde(rename = "candle4H")]
    #[strum(serialize = "candle4H")]
    Candle4h,
    /// Candlestick updates (1d)
    #[serde(rename = "candle1D")]
    #[strum(serialize = "candle1D")]
    Candle1d,
}

/// Private account channels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Display, EnumString)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum BitgetWsPrivateChannel {
    /// Order updates
    #[serde(rename = "orders")]
    #[strum(serialize = "orders")]
    Orders,
    /// Account updates
    #[serde(rename = "account")]
    #[strum(serialize = "account")]
    Account,
    /// Position updates
    #[serde(rename = "positions")]
    #[strum(serialize = "positions")]
    Positions,
}

/// Instrument type for subscriptions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Display, EnumString)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum BitgetInstType {
    /// Spot trading
    #[serde(rename = "SPOT")]
    #[strum(serialize = "SPOT")]
    Spot,
    /// USDT-margined futures
    #[serde(rename = "USDT-FUTURES")]
    #[strum(serialize = "USDT-FUTURES")]
    UsdtFutures,
    /// Coin-margined futures
    #[serde(rename = "COIN-FUTURES")]
    #[strum(serialize = "COIN-FUTURES")]
    CoinFutures,
    /// USDC-margined futures
    #[serde(rename = "USDC-FUTURES")]
    #[strum(serialize = "USDC-FUTURES")]
    UsdcFutures,
}
