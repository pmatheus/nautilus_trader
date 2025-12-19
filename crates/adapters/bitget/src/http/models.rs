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

//! Data transfer objects for deserializing Bitget HTTP API payloads.

use serde::{Deserialize, Serialize};
use ustr::Ustr;

/// Standard Bitget API response envelope.
///
/// All Bitget API responses follow this structure:
/// ```json
/// {
///   "code": "00000",
///   "msg": "success",
///   "requestTime": 1621441751927,
///   "data": { ... }
/// }
/// ```
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BitgetResponse<T> {
    /// Response code ("00000" indicates success)
    pub code: String,
    /// Response message
    pub msg: String,
    /// Request timestamp
    pub request_time: i64,
    /// Response data (None if error)
    pub data: Option<T>,
}

impl<T> BitgetResponse<T> {
    /// Returns `true` if the response indicates success.
    #[must_use]
    pub fn is_success(&self) -> bool {
        self.code == "00000"
    }
}

/// Server time response.
///
/// # References
/// - <https://www.bitget.com/api-doc/common/public/Get-Server-Time>
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BitgetServerTime {
    /// Server timestamp in milliseconds
    pub server_time: String,
}

/// Type alias for the server time response envelope.
pub type BitgetServerTimeResponse = BitgetResponse<BitgetServerTime>;

/// Spot symbol information.
///
/// # References
/// - <https://www.bitget.com/api-doc/spot/market/Get-Symbols>
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BitgetSpotSymbol {
    /// Symbol name (e.g., "BTCUSDT")
    pub symbol: Ustr,
    /// Base coin (e.g., "BTC")
    pub base_coin: Ustr,
    /// Quote coin (e.g., "USDT")
    pub quote_coin: Ustr,
    /// Minimum order quantity
    pub min_trade_amount: String,
    /// Maximum order quantity
    pub max_trade_amount: String,
    /// Price precision
    pub price_precision: String,
    /// Quantity precision
    pub quantity_precision: String,
    /// Quote precision
    pub quote_precision: String,
    /// Trading status (e.g., "online", "offline")
    pub status: String,
    /// Minimum buy quantity
    #[serde(default)]
    pub min_trade_usdt: String,
    /// Maximum buy quantity
    #[serde(default)]
    pub max_trade_usdt: String,
}

/// Response containing list of spot symbols.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BitgetSpotSymbolsData {
    /// List of spot symbols
    pub symbols: Vec<BitgetSpotSymbol>,
}

/// Type alias for the spot symbols response envelope.
pub type BitgetSpotSymbolsResponse = BitgetResponse<BitgetSpotSymbolsData>;

/// Futures symbol information.
///
/// # References
/// - <https://www.bitget.com/api-doc/contract/market/Get-All-Symbols>
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BitgetFuturesSymbol {
    /// Symbol name (e.g., "BTCUSDT")
    pub symbol: Ustr,
    /// Base coin (e.g., "BTC")
    pub base_coin: Ustr,
    /// Quote coin (e.g., "USDT")
    pub quote_coin: Ustr,
    /// Symbol type (e.g., "USDT-FUTURES", "COIN-FUTURES")
    pub symbol_type: String,
    /// Minimum order quantity
    pub size_multiplier: String,
    /// Price precision
    pub price_place: String,
    /// Quantity precision
    pub volume_place: String,
    /// Trading status
    pub symbol_status: String,
    /// Minimum trade value in USDT
    #[serde(default)]
    pub min_trade_num: String,
    /// Contract value
    #[serde(default)]
    pub contract_val: String,
}

/// Response containing list of futures symbols.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BitgetFuturesSymbolsData {
    /// List of futures symbols
    pub symbols: Vec<BitgetFuturesSymbol>,
}

/// Type alias for the futures symbols response envelope.
pub type BitgetFuturesSymbolsResponse = BitgetResponse<BitgetFuturesSymbolsData>;

/// Orderbook snapshot.
///
/// # References
/// - <https://www.bitget.com/api-doc/spot/market/Get-Orderbook>
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BitgetOrderbook {
    /// Asks (sell orders): [[price, quantity], ...]
    pub asks: Vec<[String; 2]>,
    /// Bids (buy orders): [[price, quantity], ...]
    pub bids: Vec<[String; 2]>,
    /// Timestamp in milliseconds
    pub ts: String,
}

/// Type alias for the orderbook response envelope.
pub type BitgetOrderbookResponse = BitgetResponse<BitgetOrderbook>;

/// Recent trade.
///
/// # References
/// - <https://www.bitget.com/api-doc/spot/market/Get-Recent-Trades>
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BitgetTrade {
    /// Trade ID
    pub trade_id: String,
    /// Trade price
    pub price: String,
    /// Trade quantity
    pub size: String,
    /// Trade side ("buy" or "sell")
    pub side: String,
    /// Trade timestamp in milliseconds
    pub ts: String,
    /// Symbol
    pub symbol: Ustr,
}

/// Type alias for the trades response envelope.
pub type BitgetTradesResponse = BitgetResponse<Vec<BitgetTrade>>;
