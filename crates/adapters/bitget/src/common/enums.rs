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

//! Enumerations mapping Bitget concepts onto idiomatic Nautilus variants.

use nautilus_model::enums::{LiquiditySide, OrderSide, OrderStatus, OrderType};
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, Display, EnumIter, EnumString};

/// Represents the instrument type on Bitget.
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
#[serde(rename_all = "UPPERCASE")]
pub enum BitgetInstrumentType {
    /// Spot trading.
    #[serde(rename = "SPOT")]
    Spot,
    /// USDT perpetual futures.
    #[serde(rename = "USDT-FUTURES")]
    UsdtFutures,
    /// Coin-margined perpetual futures.
    #[serde(rename = "COIN-FUTURES")]
    CoinFutures,
    /// USDC perpetual futures.
    #[serde(rename = "USDC-FUTURES")]
    UsdcFutures,
}

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
#[serde(rename_all = "lowercase")]
pub enum BitgetOrderSide {
    /// Buy side.
    Buy,
    /// Sell side.
    Sell,
}

impl From<OrderSide> for BitgetOrderSide {
    fn from(value: OrderSide) -> Self {
        match value {
            OrderSide::Buy => Self::Buy,
            OrderSide::Sell => Self::Sell,
            OrderSide::NoOrderSide => {
                panic!("NoOrderSide cannot be converted to BitgetOrderSide")
            }
        }
    }
}

impl From<BitgetOrderSide> for OrderSide {
    fn from(value: BitgetOrderSide) -> Self {
        match value {
            BitgetOrderSide::Buy => Self::Buy,
            BitgetOrderSide::Sell => Self::Sell,
        }
    }
}

/// Represents the type of order.
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
#[serde(rename_all = "lowercase")]
pub enum BitgetOrderType {
    /// Limit order.
    Limit,
    /// Market order.
    Market,
}

impl From<OrderType> for BitgetOrderType {
    fn from(value: OrderType) -> Self {
        match value {
            OrderType::Limit => Self::Limit,
            OrderType::Market => Self::Market,
            _ => panic!("Unsupported order type for Bitget: {:?}", value),
        }
    }
}

impl From<BitgetOrderType> for OrderType {
    fn from(value: BitgetOrderType) -> Self {
        match value {
            BitgetOrderType::Limit => Self::Limit,
            BitgetOrderType::Market => Self::Market,
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
#[serde(rename_all = "lowercase")]
pub enum BitgetTimeInForce {
    /// Good Till Cancel.
    #[serde(rename = "gtc")]
    GoodTilCancel,
    /// Immediate Or Cancel.
    #[serde(rename = "ioc")]
    ImmediateOrCancel,
    /// Fill Or Kill.
    #[serde(rename = "fok")]
    FillOrKill,
    /// Post Only.
    #[serde(rename = "post_only")]
    PostOnly,
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
#[serde(rename_all = "lowercase")]
pub enum BitgetOrderStatus {
    /// Order is being initialized.
    Init,
    /// Order is new and active.
    New,
    /// Order is partially filled.
    #[serde(rename = "partial_fill")]
    PartialFill,
    /// Order is fully filled.
    #[serde(rename = "full_fill")]
    FullFill,
    /// Order is cancelled.
    Cancelled,
}

impl From<BitgetOrderStatus> for OrderStatus {
    fn from(value: BitgetOrderStatus) -> Self {
        match value {
            BitgetOrderStatus::Init => Self::Initialized,
            BitgetOrderStatus::New => Self::Accepted,
            BitgetOrderStatus::PartialFill => Self::PartiallyFilled,
            BitgetOrderStatus::FullFill => Self::Filled,
            BitgetOrderStatus::Cancelled => Self::Canceled,
        }
    }
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
#[serde(rename_all = "lowercase")]
pub enum BitgetLiquiditySide {
    /// Maker (added liquidity).
    Maker,
    /// Taker (removed liquidity).
    Taker,
}

impl From<BitgetLiquiditySide> for LiquiditySide {
    fn from(value: BitgetLiquiditySide) -> Self {
        match value {
            BitgetLiquiditySide::Maker => Self::Maker,
            BitgetLiquiditySide::Taker => Self::Taker,
        }
    }
}

/// Represents the action type for order book updates.
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
#[serde(rename_all = "lowercase")]
pub enum BitgetBookAction {
    /// Full snapshot of the order book.
    Snapshot,
    /// Incremental update to the order book.
    Update,
}

/// Represents account types on Bitget.
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
#[serde(rename_all = "lowercase")]
pub enum BitgetAccountType {
    /// Spot trading account.
    Spot,
    /// Cross margin account.
    Cross,
    /// Isolated margin account.
    Isolated,
}

/// Represents futures contract types on Bitget.
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
#[serde(rename_all = "lowercase")]
pub enum BitgetContractType {
    /// USDT-margined perpetual.
    #[serde(rename = "umcbl")]
    UsdtPerpetual,
    /// Coin-margined perpetual.
    #[serde(rename = "cmcbl")]
    CoinPerpetual,
    /// USDC-margined perpetual.
    #[serde(rename = "sumcbl")]
    UsdcPerpetual,
}

/// Represents position side for futures.
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
#[serde(rename_all = "lowercase")]
pub enum BitgetPositionSide {
    /// Long position.
    Long,
    /// Short position.
    Short,
}

/// Represents margin mode for futures.
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
#[serde(rename_all = "lowercase")]
pub enum BitgetMarginMode {
    /// Cross margin mode.
    #[serde(rename = "crossed")]
    Cross,
    /// Isolated margin mode.
    #[serde(rename = "isolated")]
    Isolated,
}

/// Represents trade side for futures (open/close).
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
#[serde(rename_all = "lowercase")]
pub enum BitgetTradeSide {
    /// Open position.
    Open,
    /// Close position.
    Close,
}

/// Represents hold mode for futures positions.
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
#[serde(rename_all = "lowercase")]
pub enum BitgetHoldMode {
    /// Single-direction position mode.
    #[serde(rename = "single_hold")]
    SingleHold,
    /// Dual-direction position mode (hedge mode).
    #[serde(rename = "double_hold")]
    DoubleHold,
}
