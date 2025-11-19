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

//! Data models for Coinbase HTTP API requests and responses.

use serde::{Deserialize, Serialize};
use ustr::Ustr;

use crate::common::enums::{CoinbaseOrderStatus, CoinbaseSide, CoinbaseTimeInForce};

/// Order request.
#[derive(Clone, Debug, Serialize)]
pub struct CoinbaseOrderRequest {
    /// Product ID.
    pub product_id: String,
    /// Side (buy/sell).
    pub side: CoinbaseSide,
    /// Order type.
    #[serde(rename = "type")]
    pub order_type: String,
    /// Size (optional for market orders).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    /// Price (required for limit orders).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    /// Funds (optional for market orders).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub funds: Option<String>,
    /// Client order ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_oid: Option<String>,
    /// Time in force.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_in_force: Option<CoinbaseTimeInForce>,
    /// Post only flag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_only: Option<bool>,
    /// Stop price (for stop orders).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_price: Option<String>,
    /// Stop type (loss/entry).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<String>,
}

/// Order response.
#[derive(Clone, Debug, Deserialize)]
pub struct CoinbaseOrder {
    /// Order ID.
    pub id: String,
    /// Product ID.
    pub product_id: Ustr,
    /// Side.
    pub side: String,
    /// Order type.
    #[serde(rename = "type")]
    pub order_type: String,
    /// Size.
    pub size: String,
    /// Price (optional).
    pub price: Option<String>,
    /// Filled size.
    pub filled_size: String,
    /// Executed value.
    pub executed_value: String,
    /// Status.
    pub status: CoinbaseOrderStatus,
    /// Settled.
    pub settled: bool,
    /// Created at.
    pub created_at: String,
    /// Client order ID (optional).
    pub client_oid: Option<String>,
    /// Fill fees.
    pub fill_fees: String,
    /// Time in force.
    pub time_in_force: Option<String>,
    /// Post only.
    pub post_only: bool,
}
