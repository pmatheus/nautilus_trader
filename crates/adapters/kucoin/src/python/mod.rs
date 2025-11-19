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

//! Python bindings for the Kucoin adapter.

use pyo3::prelude::*;

use crate::{
    common::enums::{KucoinOrderSide, KucoinOrderType, KucoinProductType, KucoinTimeInForce},
    websocket::client::KucoinWebSocketClient,
};

/// Register Kucoin Python module.
#[pymodule]
pub fn kucoin(_: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Register enums
    m.add_class::<KucoinOrderSide>()?;
    m.add_class::<KucoinOrderType>()?;
    m.add_class::<KucoinTimeInForce>()?;
    m.add_class::<KucoinProductType>()?;

    // Register clients
    m.add_class::<KucoinWebSocketClient>()?;

    Ok(())
}
