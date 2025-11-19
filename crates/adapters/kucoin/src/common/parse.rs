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

//! Parsing utilities for converting Kucoin data to Nautilus types.

use nautilus_model::enums::{OrderSide, OrderType, TimeInForce};

use super::enums::{KucoinOrderSide, KucoinOrderType, KucoinTimeInForce};

/// Converts a Nautilus [`OrderSide`] to a Kucoin [`KucoinOrderSide`].
#[must_use]
pub fn to_kucoin_order_side(side: OrderSide) -> KucoinOrderSide {
    match side {
        OrderSide::Buy => KucoinOrderSide::Buy,
        OrderSide::Sell => KucoinOrderSide::Sell,
        _ => panic!("Unsupported order side: {side:?}"),
    }
}

/// Converts a Kucoin [`KucoinOrderSide`] to a Nautilus [`OrderSide`].
#[must_use]
pub fn from_kucoin_order_side(side: KucoinOrderSide) -> OrderSide {
    match side {
        KucoinOrderSide::Buy => OrderSide::Buy,
        KucoinOrderSide::Sell => OrderSide::Sell,
    }
}

/// Converts a Nautilus [`OrderType`] to a Kucoin [`KucoinOrderType`].
#[must_use]
pub fn to_kucoin_order_type(order_type: OrderType) -> KucoinOrderType {
    match order_type {
        OrderType::Limit => KucoinOrderType::Limit,
        OrderType::Market => KucoinOrderType::Market,
        OrderType::StopLimit => KucoinOrderType::LimitStop,
        OrderType::StopMarket => KucoinOrderType::MarketStop,
        _ => panic!("Unsupported order type: {order_type:?}"),
    }
}

/// Converts a Kucoin [`KucoinOrderType`] to a Nautilus [`OrderType`].
#[must_use]
pub fn from_kucoin_order_type(order_type: KucoinOrderType) -> OrderType {
    match order_type {
        KucoinOrderType::Limit => OrderType::Limit,
        KucoinOrderType::Market => OrderType::Market,
        KucoinOrderType::LimitStop => OrderType::StopLimit,
        KucoinOrderType::MarketStop => OrderType::StopMarket,
    }
}

/// Converts a Nautilus [`TimeInForce`] to a Kucoin [`KucoinTimeInForce`].
#[must_use]
pub fn to_kucoin_time_in_force(time_in_force: TimeInForce) -> KucoinTimeInForce {
    match time_in_force {
        TimeInForce::GTC => KucoinTimeInForce::GTC,
        TimeInForce::GTD => KucoinTimeInForce::GTT,
        TimeInForce::IOC => KucoinTimeInForce::IOC,
        TimeInForce::FOK => KucoinTimeInForce::FOK,
        _ => panic!("Unsupported time in force: {time_in_force:?}"),
    }
}

/// Converts a Kucoin [`KucoinTimeInForce`] to a Nautilus [`TimeInForce`].
#[must_use]
pub fn from_kucoin_time_in_force(time_in_force: KucoinTimeInForce) -> TimeInForce {
    match time_in_force {
        KucoinTimeInForce::GTC => TimeInForce::GTC,
        KucoinTimeInForce::GTT => TimeInForce::GTD,
        KucoinTimeInForce::IOC => TimeInForce::IOC,
        KucoinTimeInForce::FOK => TimeInForce::FOK,
    }
}
