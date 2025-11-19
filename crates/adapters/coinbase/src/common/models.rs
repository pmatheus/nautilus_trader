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

//! Data models representing Coinbase API payloads consumed by the adapter.

use serde::{Deserialize, Serialize};
use ustr::Ustr;

/// Represents a product (trading pair) on Coinbase.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CoinbaseProduct {
    /// Product ID, e.g., "BTC-USD".
    pub id: Ustr,
    /// Base currency, e.g., "BTC".
    pub base_currency: Ustr,
    /// Quote currency, e.g., "USD".
    pub quote_currency: Ustr,
    /// Base currency minimum size.
    pub base_min_size: String,
    /// Base currency maximum size.
    pub base_max_size: String,
    /// Quote currency increment.
    pub quote_increment: String,
    /// Base currency increment.
    pub base_increment: String,
    /// Display name.
    pub display_name: String,
    /// Minimum market funds.
    pub min_market_funds: String,
    /// Maximum market funds.
    pub max_market_funds: String,
    /// Margin enabled.
    pub margin_enabled: bool,
    /// Post only.
    pub post_only: bool,
    /// Limit only.
    pub limit_only: bool,
    /// Cancel only.
    pub cancel_only: bool,
    /// Trading disabled.
    pub trading_disabled: bool,
    /// Status.
    pub status: String,
    /// Status message.
    pub status_message: String,
}

/// Represents an account on Coinbase.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CoinbaseAccount {
    /// Account ID.
    pub id: String,
    /// Currency.
    pub currency: Ustr,
    /// Available balance.
    pub balance: String,
    /// Held balance (in orders).
    pub hold: String,
    /// Available balance for trading.
    pub available: String,
    /// Profile ID.
    pub profile_id: String,
    /// Trading enabled.
    pub trading_enabled: bool,
}

/// Represents a currency on Coinbase.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CoinbaseCurrency {
    /// Currency ID, e.g., "BTC".
    pub id: Ustr,
    /// Currency name.
    pub name: String,
    /// Minimum withdrawal amount.
    pub min_size: String,
    /// Status.
    pub status: String,
    /// Message.
    pub message: String,
    /// Max precision.
    pub max_precision: String,
    /// Convertible to.
    pub convertible_to: Vec<String>,
    /// Details.
    pub details: CoinbaseCurrencyDetails,
}

/// Currency details.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CoinbaseCurrencyDetails {
    /// Type (e.g., "crypto").
    #[serde(rename = "type")]
    pub currency_type: Option<String>,
    /// Symbol.
    pub symbol: Option<String>,
    /// Network confirmations.
    pub network_confirmations: Option<u32>,
    /// Sort order.
    pub sort_order: Option<u32>,
    /// Crypto address link.
    pub crypto_address_link: Option<String>,
    /// Crypto transaction link.
    pub crypto_transaction_link: Option<String>,
    /// Push payment methods.
    pub push_payment_methods: Option<Vec<String>>,
    /// Group types.
    pub group_types: Option<Vec<String>>,
    /// Display name.
    pub display_name: Option<String>,
    /// Processing time (seconds).
    pub processing_time_seconds: Option<f64>,
    /// Min withdrawal amount.
    pub min_withdrawal_amount: Option<f64>,
    /// Max withdrawal amount.
    pub max_withdrawal_amount: Option<f64>,
}

/// WebSocket subscription message.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CoinbaseSubscription {
    /// Message type.
    #[serde(rename = "type")]
    pub msg_type: String,
    /// Product IDs to subscribe to.
    pub product_ids: Vec<String>,
    /// Channels to subscribe to.
    pub channels: Vec<String>,
    /// Authentication (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
    /// API key (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// Passphrase (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passphrase: Option<String>,
    /// Timestamp (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
}

/// Error response from Coinbase API.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CoinbaseError {
    /// Error message.
    pub message: String,
}

/// Represents a trade (fill) on Coinbase.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CoinbaseFill {
    /// Trade ID.
    pub trade_id: u64,
    /// Product ID.
    pub product_id: Ustr,
    /// Order ID.
    pub order_id: String,
    /// User ID.
    pub user_id: String,
    /// Profile ID.
    pub profile_id: String,
    /// Liquidity (M for maker, T for taker).
    pub liquidity: String,
    /// Price.
    pub price: String,
    /// Size.
    pub size: String,
    /// Fee.
    pub fee: String,
    /// Created at (ISO 8601).
    pub created_at: String,
    /// Side.
    pub side: String,
    /// Settled.
    pub settled: bool,
    /// USD volume.
    pub usd_volume: Option<String>,
}
