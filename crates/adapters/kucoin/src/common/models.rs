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

//! Kucoin common data models.

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// Kucoin instrument (symbol) information.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KucoinInstrument {
    /// Symbol identifier (e.g., "BTC-USDT")
    pub symbol: String,
    /// Base currency (e.g., "BTC")
    pub base_currency: String,
    /// Quote currency (e.g., "USDT")
    pub quote_currency: String,
    /// Fee currency
    pub fee_currency: String,
    /// Market type (e.g., "SPOT")
    pub market: String,
    /// Base min size
    pub base_min_size: Decimal,
    /// Quote min size
    pub quote_min_size: Decimal,
    /// Base max size
    pub base_max_size: Decimal,
    /// Quote max size
    pub quote_max_size: Decimal,
    /// Base increment
    pub base_increment: Decimal,
    /// Quote increment
    pub quote_increment: Decimal,
    /// Price increment
    pub price_increment: Decimal,
    /// Price limit rate (percentage for price limits)
    pub price_limit_rate: Decimal,
    /// Minimum funds for order
    pub min_funds: Option<Decimal>,
    /// Whether the symbol is enabled for trading
    pub enable_trading: bool,
    /// Whether the symbol is margin enabled
    #[serde(default)]
    pub is_margin_enabled: bool,
}

/// WebSocket token response from Kucoin.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KucoinWsTokenResponse {
    /// Token for WebSocket connection
    pub token: String,
    /// List of available WebSocket servers
    pub instance_servers: Vec<KucoinWsServer>,
}

/// WebSocket server information.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KucoinWsServer {
    /// WebSocket endpoint URL
    pub endpoint: String,
    /// Encryption protocol
    pub encrypt: bool,
    /// Protocol version
    pub protocol: String,
    /// Ping interval in milliseconds
    pub ping_interval: u64,
    /// Ping timeout in milliseconds
    pub ping_timeout: u64,
}

/// Kucoin futures contract (instrument) information.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KucoinFuturesContract {
    /// Symbol identifier (e.g., "XBTUSDTM")
    pub symbol: String,
    /// Root symbol (e.g., "USDT")
    pub root_symbol: String,
    /// Contract type (e.g., "FFWCSX")
    #[serde(rename = "type")]
    pub contract_type: String,
    /// First open date
    pub first_open_date: i64,
    /// Expiry date (None for perpetuals)
    pub expire_date: Option<i64>,
    /// Settlement date (None for perpetuals)
    pub settle_date: Option<i64>,
    /// Base currency (e.g., "XBT")
    pub base_currency: String,
    /// Quote currency (e.g., "USDT")
    pub quote_currency: String,
    /// Settlement currency (e.g., "USDT")
    pub settlement_currency: String,
    /// Maximum order quantity
    pub max_order_qty: i64,
    /// Maximum price
    pub max_price: Decimal,
    /// Lot size (minimum order quantity)
    pub lot_size: i64,
    /// Tick size (price increment)
    pub tick_size: Decimal,
    /// Index price tick size
    pub index_price_tick_size: Decimal,
    /// Multiplier (contract value)
    pub multiplier: Decimal,
    /// Initial margin rate
    pub initial_margin: Decimal,
    /// Maintenance margin rate
    pub maintenance_margin: Decimal,
    /// Maximum risk limit
    pub max_risk_limit: i64,
    /// Minimum risk limit
    pub min_risk_limit: i64,
    /// Risk step
    pub risk_step: i64,
    /// Maker fee rate
    pub maker_fee_rate: Decimal,
    /// Taker fee rate
    pub taker_fee_rate: Decimal,
    /// Funding base symbol 1
    #[serde(default)]
    pub funding_base_symbol_1: Option<String>,
    /// Funding quote symbol 1
    #[serde(default)]
    pub funding_quote_symbol_1: Option<String>,
    /// Funding rate symbol
    #[serde(default)]
    pub funding_rate_symbol: Option<String>,
    /// Index symbol
    pub index_symbol: String,
    /// Settlement symbol
    pub settlement_symbol: String,
    /// Status (e.g., "Open")
    pub status: String,
    /// Funding fee rate
    #[serde(default)]
    pub funding_fee_rate: Option<Decimal>,
    /// Predicted funding fee rate
    #[serde(default)]
    pub predicted_funding_fee_rate: Option<Decimal>,
    /// Open interest
    #[serde(default)]
    pub open_interest: Option<String>,
    /// Turnover of 24 hours
    #[serde(default)]
    pub turnover_of24h: Option<Decimal>,
    /// Volume of 24 hours
    #[serde(default)]
    pub volume_of24h: Option<Decimal>,
    /// Mark price
    #[serde(default)]
    pub mark_price: Option<Decimal>,
    /// Index price
    #[serde(default)]
    pub index_price: Option<Decimal>,
    /// Last traded price
    #[serde(default)]
    pub last_trade_price: Option<Decimal>,
    /// Next funding rate time
    #[serde(default)]
    pub next_funding_rate_time: Option<i64>,
    /// Maximum leverage
    #[serde(default)]
    pub max_leverage: Option<Decimal>,
    /// Whether the contract is enabled for trading
    #[serde(default)]
    pub is_inverse: bool,
    /// Whether the contract is quanto
    #[serde(default)]
    pub is_quanto: bool,
    /// Whether the contract is inverse
    #[serde(default)]
    pub is_perpetual: bool,
}
