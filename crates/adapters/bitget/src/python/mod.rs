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

//! Python bindings for the Bitget adapter.
//!
//! This module provides PyO3 bindings to expose Rust functionality to Python.

use pyo3::prelude::*;

/// Registers the Bitget adapter types with the `nautilus_pyo3.bitget` module.
pub fn register_bitget(py: Python, parent: &Bound<'_, PyModule>) -> PyResult<()> {
    let bitget_module = PyModule::new(py, "bitget")?;

    // Register enums when implemented
    // bitget_module.add_class::<BitgetEnvironment>()?;
    // bitget_module.add_class::<BitgetProductType>()?;
    // bitget_module.add_class::<BitgetMarginMode>()?;
    // bitget_module.add_class::<BitgetPositionMode>()?;

    parent.add_submodule(&bitget_module)?;

    Ok(())
}
