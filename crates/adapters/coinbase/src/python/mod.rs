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

//! Python bindings for Coinbase adapter.

use pyo3::prelude::*;

/// Loaded as `nautilus_trader.core.nautilus_pyo3.coinbase`.
#[pymodule]
pub fn coinbase(_: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Register enums
    m.add_class::<crate::common::enums::CoinbaseSide>()?;
    m.add_class::<crate::common::enums::CoinbaseOrderType>()?;
    m.add_class::<crate::common::enums::CoinbaseTimeInForce>()?;
    m.add_class::<crate::common::enums::CoinbaseOrderStatus>()?;
    m.add_class::<crate::common::enums::CoinbaseStopType>()?;
    m.add_class::<crate::common::enums::CoinbaseLiquidity>()?;
    m.add_class::<crate::common::enums::CoinbaseMessageType>()?;
    m.add_class::<crate::common::enums::CoinbaseDoneReason>()?;

    Ok(())
}
