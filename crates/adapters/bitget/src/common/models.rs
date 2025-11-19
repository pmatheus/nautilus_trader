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

//! Data models for Bitget API responses.

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use super::enums::BitgetInstrumentType;

/// Represents a trading instrument on Bitget.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BitgetInstrument {
    /// Symbol identifier (e.g., "BTCUSDT").
    pub symbol: String,
    /// Base currency (e.g., "BTC").
    pub base_coin: String,
    /// Quote currency (e.g., "USDT").
    pub quote_coin: String,
    /// Minimum order quantity.
    pub min_trade_amount: Decimal,
    /// Maximum order quantity.
    pub max_trade_amount: Decimal,
    /// Price tick size.
    pub price_scale: u32,
    /// Quantity tick size.
    pub quantity_scale: u32,
    /// Trading status: online, offline, pause.
    pub status: String,
}

/// Represents instrument information from Bitget API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BitgetInstrumentInfo {
    /// Instrument type.
    pub inst_type: BitgetInstrumentType,
    /// List of instruments.
    pub instruments: Vec<BitgetInstrument>,
}

/// Generic API response wrapper.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BitgetApiResponse<T> {
    /// Response code (00000 = success).
    pub code: String,
    /// Response message.
    pub msg: String,
    /// Request timestamp.
    #[serde(rename = "requestTime")]
    pub request_time: u64,
    /// Response data.
    pub data: T,
}

/// Represents a futures contract on Bitget.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BitgetFuturesContract {
    /// Symbol identifier (e.g., "BTCUSDT").
    pub symbol: String,
    /// Base coin (e.g., "BTC").
    pub base_coin: String,
    /// Quote coin (e.g., "USDT").
    pub quote_coin: String,
    /// Contract value (for coin-margined).
    #[serde(default)]
    pub size: Option<Decimal>,
    /// Minimum order size.
    pub min_trade_num: Decimal,
    /// Maximum order size.
    pub max_trade_num: Decimal,
    /// Price precision.
    pub price_place: u32,
    /// Volume precision.
    pub volume_place: u32,
    /// Tick size (price).
    pub price_end_step: Decimal,
    /// Volume multiplier.
    pub volume_multiplier: Decimal,
    /// Minimum leverage.
    pub min_lever: Decimal,
    /// Maximum leverage.
    pub max_lever: Decimal,
    /// Contract status: normal, maintain, offline.
    pub status: String,
}

/// Represents a futures position on Bitget.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BitgetPosition {
    /// Position ID.
    pub position_id: String,
    /// Symbol.
    pub symbol: String,
    /// Margin coin.
    pub margin_coin: String,
    /// Margin mode: crossed, isolated.
    pub margin_mode: String,
    /// Hold side: long, short.
    pub hold_side: String,
    /// Position quantity.
    pub total: Decimal,
    /// Available to close.
    pub available: Decimal,
    /// Locked quantity.
    pub locked: Decimal,
    /// Average open price.
    pub average_open_price: Decimal,
    /// Current leverage.
    pub leverage: Decimal,
    /// Unrealized profit/loss.
    pub unrealized_pl: Decimal,
    /// Liquidation price.
    pub liquidation_price: Option<Decimal>,
    /// Margin size.
    pub margin_size: Decimal,
    /// Position margin.
    pub margin: Decimal,
    /// Creation time.
    pub ctime: String,
    /// Update time.
    pub utime: String,
}

/// Represents account information for futures.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BitgetFuturesAccount {
    /// Margin coin.
    pub margin_coin: String,
    /// Account equity.
    pub equity: Decimal,
    /// Available balance.
    pub available: Decimal,
    /// Total position margin.
    pub used_margin: Decimal,
    /// Unrealized PnL.
    pub unrealized_pl: Decimal,
    /// Account risk rate.
    #[serde(default)]
    pub account_risk_rate: Option<Decimal>,
}

/// Represents funding rate data.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BitgetFundingRate {
    /// Symbol.
    pub symbol: String,
    /// Funding rate.
    pub funding_rate: Decimal,
    /// Settlement time (timestamp).
    pub funding_time: i64,
}
