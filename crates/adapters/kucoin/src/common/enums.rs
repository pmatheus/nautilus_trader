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

//! Kucoin enumeration types.

use serde::{Deserialize, Serialize};
use strum::{AsRefStr, Display, EnumString, FromRepr};

/// Kucoin order side.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    Display,
    AsRefStr,
    EnumString,
    FromRepr,
)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
#[repr(C)]
pub enum KucoinOrderSide {
    /// Buy order
    Buy,
    /// Sell order
    Sell,
}

/// Kucoin order type.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    Display,
    AsRefStr,
    EnumString,
    FromRepr,
)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
#[repr(C)]
pub enum KucoinOrderType {
    /// Limit order
    Limit,
    /// Market order
    Market,
    /// Limit stop order
    #[serde(rename = "limit_stop")]
    #[strum(serialize = "limit_stop")]
    LimitStop,
    /// Market stop order
    #[serde(rename = "market_stop")]
    #[strum(serialize = "market_stop")]
    MarketStop,
}

/// Kucoin time in force.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    Display,
    AsRefStr,
    EnumString,
    FromRepr,
)]
#[repr(C)]
pub enum KucoinTimeInForce {
    /// Good till canceled
    #[serde(rename = "GTC")]
    #[strum(serialize = "GTC")]
    GTC,
    /// Good till time
    #[serde(rename = "GTT")]
    #[strum(serialize = "GTT")]
    GTT,
    /// Immediate or cancel
    #[serde(rename = "IOC")]
    #[strum(serialize = "IOC")]
    IOC,
    /// Fill or kill
    #[serde(rename = "FOK")]
    #[strum(serialize = "FOK")]
    FOK,
}

/// Kucoin order status.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    Display,
    AsRefStr,
    EnumString,
    FromRepr,
)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
#[repr(C)]
pub enum KucoinOrderStatus {
    /// Order is active
    Active,
    /// Order is done (filled or canceled)
    Done,
}

/// Kucoin stop order type.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    Display,
    AsRefStr,
    EnumString,
    FromRepr,
)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
#[repr(C)]
pub enum KucoinStopType {
    /// Entry stop order
    Entry,
    /// Loss stop order
    Loss,
}

/// Kucoin trade type.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    Display,
    AsRefStr,
    EnumString,
    FromRepr,
)]
#[repr(C)]
pub enum KucoinTradeType {
    /// SPOT trading
    #[serde(rename = "TRADE")]
    #[strum(serialize = "TRADE")]
    Trade,
    /// Margin trading
    #[serde(rename = "MARGIN_TRADE")]
    #[strum(serialize = "MARGIN_TRADE")]
    MarginTrade,
}

/// Kucoin account type.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    Display,
    AsRefStr,
    EnumString,
    FromRepr,
)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
#[repr(C)]
pub enum KucoinAccountType {
    /// Main account
    Main,
    /// Trading account
    Trade,
    /// Margin account
    Margin,
}

/// Kucoin product type.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    Display,
    AsRefStr,
    EnumString,
    FromRepr,
)]
#[cfg_attr(
    feature = "python",
    pyo3::pyclass(module = "nautilus_trader.core.nautilus_pyo3.adapters", eq, eq_int)
)]
#[repr(C)]
pub enum KucoinProductType {
    /// Spot trading
    #[serde(rename = "SPOT")]
    #[strum(serialize = "SPOT")]
    Spot = 0,
    /// Futures trading (USDT-margined perpetuals and futures)
    #[serde(rename = "FUTURES")]
    #[strum(serialize = "FUTURES")]
    Futures = 1,
}

/// Kucoin WebSocket event type.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    Display,
    AsRefStr,
    EnumString,
    FromRepr,
)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
#[repr(C)]
pub enum KucoinWsEventType {
    /// Welcome message
    Welcome,
    /// Subscription acknowledgment
    Ack,
    /// Subscription error
    Error,
    /// Pong response
    Pong,
    /// Message event
    Message,
}
