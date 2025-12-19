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

//! Enumerations that model Bitget string/int enums across HTTP and WebSocket payloads.

use serde::{Deserialize, Serialize};
use strum::{AsRefStr, EnumIter, EnumString};

/// Environments supported by the Bitget API stack.
#[derive(
    Copy,
    Clone,
    Debug,
    strum::Display,
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
#[cfg_attr(
    feature = "python",
    pyo3::pyclass(eq, eq_int, module = "nautilus_trader.core.nautilus_pyo3.bitget")
)]
pub enum BitgetEnvironment {
    /// Live trading environment.
    Mainnet,
    /// Testnet environment for spot/derivatives.
    Testnet,
}

/// Product types supported by the Bitget v2 API.
#[derive(
    Copy,
    Clone,
    Debug,
    strum::Display,
    Default,
    PartialEq,
    Eq,
    Hash,
    AsRefStr,
    EnumIter,
    EnumString,
    Serialize,
    Deserialize,
)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[cfg_attr(
    feature = "python",
    pyo3::pyclass(eq, eq_int, module = "nautilus_trader.core.nautilus_pyo3.bitget")
)]
pub enum BitgetProductType {
    /// Spot trading.
    #[default]
    Spot,
    /// USDT-margined futures.
    UsdtFutures,
    /// USDC-margined futures.
    UsdcFutures,
    /// Coin-margined futures.
    CoinFutures,
}

impl BitgetProductType {
    /// Returns the canonical uppercase identifier used for REST/WS routes.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Spot => "SPOT",
            Self::UsdtFutures => "USDT-FUTURES",
            Self::UsdcFutures => "USDC-FUTURES",
            Self::CoinFutures => "COIN-FUTURES",
        }
    }

    /// Returns `true` if the product is a spot instrument.
    #[must_use]
    pub fn is_spot(self) -> bool {
        matches!(self, Self::Spot)
    }

    /// Returns `true` if the product is a futures contract.
    #[must_use]
    pub fn is_futures(self) -> bool {
        matches!(
            self,
            Self::UsdtFutures | Self::UsdcFutures | Self::CoinFutures
        )
    }
}

/// Margin mode used by Bitget for futures trading.
#[derive(
    Clone,
    Copy,
    Debug,
    strum::Display,
    Eq,
    PartialEq,
    Hash,
    AsRefStr,
    EnumIter,
    EnumString,
    Serialize,
    Deserialize,
)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
#[cfg_attr(
    feature = "python",
    pyo3::pyclass(eq, eq_int, module = "nautilus_trader.core.nautilus_pyo3.bitget")
)]
pub enum BitgetMarginMode {
    /// Crossed margin mode.
    Crossed,
    /// Isolated margin mode.
    Isolated,
}

/// Position mode as supported by the Bitget v2 API.
#[derive(
    Clone,
    Copy,
    Debug,
    strum::Display,
    Eq,
    PartialEq,
    Hash,
    AsRefStr,
    EnumIter,
    EnumString,
    Serialize,
    Deserialize,
)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
#[cfg_attr(
    feature = "python",
    pyo3::pyclass(eq, eq_int, module = "nautilus_trader.core.nautilus_pyo3.bitget")
)]
pub enum BitgetPositionMode {
    /// One-way position mode.
    OneWayMode,
    /// Hedge mode (dual-side positions).
    HedgeMode,
}
