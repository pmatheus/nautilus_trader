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

//! Enumerations mapping Coinbase concepts onto idiomatic Nautilus variants.

use nautilus_model::enums::{LiquiditySide, OrderSide, OrderStatus, OrderType};
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, Display, EnumIter, EnumString};

/// Represents the side of an order or trade (Buy/Sell).
#[derive(
    Copy,
    Clone,
    Debug,
    Display,
    PartialEq,
    Eq,
    Hash,
    AsRefStr,
    EnumIter,
    EnumString,
    Serialize,
    Deserialize,
)]
#[cfg_attr(feature = "python", pyo3::pyclass(module = "nautilus_trader.core.nautilus_pyo3.coinbase"))]
#[serde(rename_all = "lowercase")]
pub enum CoinbaseSide {
    /// Buy side of a trade or order.
    Buy,
    /// Sell side of a trade or order.
    Sell,
}

impl From<OrderSide> for CoinbaseSide {
    fn from(value: OrderSide) -> Self {
        match value {
            OrderSide::Buy => Self::Buy,
            OrderSide::Sell => Self::Sell,
            OrderSide::NoOrderSide => panic!("NoOrderSide cannot be converted to CoinbaseSide"),
        }
    }
}

impl From<CoinbaseSide> for OrderSide {
    fn from(value: CoinbaseSide) -> Self {
        match value {
            CoinbaseSide::Buy => Self::Buy,
            CoinbaseSide::Sell => Self::Sell,
        }
    }
}

/// Represents the type of an order.
#[derive(
    Copy,
    Clone,
    Debug,
    Display,
    PartialEq,
    Eq,
    Hash,
    AsRefStr,
    EnumIter,
    EnumString,
    Serialize,
    Deserialize,
)]
#[cfg_attr(feature = "python", pyo3::pyclass(module = "nautilus_trader.core.nautilus_pyo3.coinbase"))]
#[serde(rename_all = "lowercase")]
pub enum CoinbaseOrderType {
    /// Market order.
    Market,
    /// Limit order.
    Limit,
    /// Stop order (stop-loss or stop-entry).
    Stop,
}

impl From<OrderType> for CoinbaseOrderType {
    fn from(value: OrderType) -> Self {
        match value {
            OrderType::Market => Self::Market,
            OrderType::Limit => Self::Limit,
            OrderType::StopMarket | OrderType::StopLimit => Self::Stop,
            _ => panic!("Unsupported OrderType for Coinbase: {value:?}"),
        }
    }
}

/// Represents the time in force for an order.
#[derive(
    Copy,
    Clone,
    Debug,
    Display,
    PartialEq,
    Eq,
    Hash,
    AsRefStr,
    EnumIter,
    EnumString,
    Serialize,
    Deserialize,
)]
#[cfg_attr(feature = "python", pyo3::pyclass(module = "nautilus_trader.core.nautilus_pyo3.coinbase"))]
#[serde(rename_all = "UPPERCASE")]
pub enum CoinbaseTimeInForce {
    /// Good Till Canceled.
    #[serde(rename = "GTC")]
    Gtc,
    /// Good Till Time (expires at specific time).
    #[serde(rename = "GTT")]
    Gtt,
    /// Immediate or Cancel.
    #[serde(rename = "IOC")]
    Ioc,
    /// Fill or Kill.
    #[serde(rename = "FOK")]
    Fok,
}

/// Represents the status of an order.
#[derive(
    Copy,
    Clone,
    Debug,
    Display,
    PartialEq,
    Eq,
    Hash,
    AsRefStr,
    EnumIter,
    EnumString,
    Serialize,
    Deserialize,
)]
#[cfg_attr(feature = "python", pyo3::pyclass(module = "nautilus_trader.core.nautilus_pyo3.coinbase"))]
#[serde(rename_all = "lowercase")]
pub enum CoinbaseOrderStatus {
    /// Order is open and active.
    Open,
    /// Order is pending (not yet active).
    Pending,
    /// Order is active.
    Active,
    /// Order is done (filled or cancelled).
    Done,
    /// Order has been rejected.
    Rejected,
}

impl From<CoinbaseOrderStatus> for OrderStatus {
    fn from(value: CoinbaseOrderStatus) -> Self {
        match value {
            CoinbaseOrderStatus::Pending => Self::Submitted,
            CoinbaseOrderStatus::Open | CoinbaseOrderStatus::Active => Self::Accepted,
            CoinbaseOrderStatus::Done => Self::Filled, // May need to check fill status
            CoinbaseOrderStatus::Rejected => Self::Rejected,
        }
    }
}

/// Represents the stop type for stop orders.
#[derive(
    Copy,
    Clone,
    Debug,
    Display,
    PartialEq,
    Eq,
    Hash,
    AsRefStr,
    EnumIter,
    EnumString,
    Serialize,
    Deserialize,
)]
#[cfg_attr(feature = "python", pyo3::pyclass(module = "nautilus_trader.core.nautilus_pyo3.coinbase"))]
#[serde(rename_all = "lowercase")]
pub enum CoinbaseStopType {
    /// Stop loss order.
    Loss,
    /// Stop entry order.
    Entry,
}

/// Represents the liquidity side of a trade (maker/taker).
#[derive(
    Copy,
    Clone,
    Debug,
    Display,
    PartialEq,
    Eq,
    Hash,
    AsRefStr,
    EnumIter,
    EnumString,
    Serialize,
    Deserialize,
)]
#[cfg_attr(feature = "python", pyo3::pyclass(module = "nautilus_trader.core.nautilus_pyo3.coinbase"))]
#[serde(rename_all = "UPPERCASE")]
pub enum CoinbaseLiquidity {
    /// Maker (provided liquidity).
    #[serde(rename = "M")]
    Maker,
    /// Taker (took liquidity).
    #[serde(rename = "T")]
    Taker,
}

impl From<CoinbaseLiquidity> for LiquiditySide {
    fn from(value: CoinbaseLiquidity) -> Self {
        match value {
            CoinbaseLiquidity::Maker => Self::Maker,
            CoinbaseLiquidity::Taker => Self::Taker,
        }
    }
}

/// Represents the type of message received via WebSocket.
#[derive(
    Copy,
    Clone,
    Debug,
    Display,
    PartialEq,
    Eq,
    Hash,
    AsRefStr,
    EnumIter,
    EnumString,
    Serialize,
    Deserialize,
)]
#[cfg_attr(feature = "python", pyo3::pyclass(module = "nautilus_trader.core.nautilus_pyo3.coinbase"))]
#[serde(rename_all = "lowercase")]
pub enum CoinbaseMessageType {
    /// Subscription confirmation.
    Subscriptions,
    /// Heartbeat message.
    Heartbeat,
    /// Status message.
    Status,
    /// Ticker update.
    Ticker,
    /// Level 2 snapshot.
    Snapshot,
    /// Level 2 update.
    L2update,
    /// Trade match.
    Match,
    /// Last match (final trade in batch).
    #[serde(rename = "last_match")]
    LastMatch,
    /// Order received.
    Received,
    /// Order opened.
    Open,
    /// Order done (filled/cancelled).
    Done,
    /// Order changed.
    Change,
    /// Order activated.
    Activate,
    /// Error message.
    Error,
}

/// Represents the reason why an order was done (filled/cancelled).
#[derive(
    Copy,
    Clone,
    Debug,
    Display,
    PartialEq,
    Eq,
    Hash,
    AsRefStr,
    EnumIter,
    EnumString,
    Serialize,
    Deserialize,
)]
#[cfg_attr(feature = "python", pyo3::pyclass(module = "nautilus_trader.core.nautilus_pyo3.coinbase"))]
#[serde(rename_all = "lowercase")]
pub enum CoinbaseDoneReason {
    /// Order was filled.
    Filled,
    /// Order was cancelled.
    Canceled,
}
