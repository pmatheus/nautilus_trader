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

//! Python bindings for the Iceberg integration.

use nautilus_core::python::to_pyvalue_err;
use pyo3::prelude::*;

use crate::config::{IcebergConfig, IcebergCredentials};
use crate::sink::IcebergSink;
use crate::types::{OrderAction, OrderSide, OrderbookL3Event};

/// Python wrapper for `IcebergCredentials`.
#[pyclass(module = "nautilus_trader.core.nautilus_pyo3.iceberg")]
#[derive(Clone)]
pub struct PyIcebergCredentials {
    inner: IcebergCredentials,
}

#[pymethods]
impl PyIcebergCredentials {
    /// Creates new Iceberg credentials.
    ///
    /// Parameters
    /// ----------
    /// api_key : str
    ///     The Supabase API key or service role key.
    ///
    /// Returns
    /// -------
    /// PyIcebergCredentials
    ///
    #[new]
    fn new(api_key: String) -> Self {
        Self {
            inner: IcebergCredentials::new(api_key),
        }
    }

    /// Creates credentials from environment variables.
    ///
    /// Reads from `SUPABASE_API_KEY` environment variable.
    ///
    /// Returns
    /// -------
    /// PyIcebergCredentials
    ///
    /// Raises
    /// ------
    /// ValueError
    ///     If the environment variable is not set.
    ///
    #[staticmethod]
    fn from_env() -> PyResult<Self> {
        let inner = IcebergCredentials::from_env().map_err(to_pyvalue_err)?;
        Ok(Self { inner })
    }

    fn __repr__(&self) -> String {
        format!("PyIcebergCredentials(api_key='***')")
    }
}

/// Python wrapper for `IcebergConfig`.
#[pyclass(module = "nautilus_trader.core.nautilus_pyo3.iceberg")]
#[derive(Clone)]
pub struct PyIcebergConfig {
    inner: IcebergConfig,
}

#[pymethods]
impl PyIcebergConfig {
    /// Creates a new Iceberg configuration.
    ///
    /// Parameters
    /// ----------
    /// project_ref : str
    ///     The Supabase project reference ID.
    /// credentials : PyIcebergCredentials
    ///     The authentication credentials.
    /// warehouse_name : str, optional
    ///     The warehouse name (default: "warehouse").
    /// batch_size : int, optional
    ///     Batch size for buffering events (default: 10000).
    /// flush_interval_secs : int, optional
    ///     Flush interval in seconds (default: 60).
    ///
    /// Returns
    /// -------
    /// PyIcebergConfig
    ///
    /// Raises
    /// ------
    /// ValueError
    ///     If the configuration is invalid.
    ///
    #[new]
    #[pyo3(signature = (project_ref, credentials, warehouse_name=None, batch_size=None, flush_interval_secs=None))]
    fn new(
        project_ref: String,
        credentials: PyIcebergCredentials,
        warehouse_name: Option<String>,
        batch_size: Option<usize>,
        flush_interval_secs: Option<u64>,
    ) -> PyResult<Self> {
        let mut builder = IcebergConfig::builder()
            .project_ref(project_ref)
            .credentials(credentials.inner);

        if let Some(name) = warehouse_name {
            builder = builder.warehouse_name(name);
        }

        if let Some(size) = batch_size {
            builder = builder.batch_size(size);
        }

        if let Some(interval) = flush_interval_secs {
            builder = builder.flush_interval_secs(interval);
        }

        let inner = builder.build().map_err(to_pyvalue_err)?;
        Ok(Self { inner })
    }

    /// Returns the S3 endpoint URL.
    ///
    /// Returns
    /// -------
    /// str
    ///
    #[getter]
    fn s3_endpoint(&self) -> String {
        self.inner.s3_endpoint()
    }

    /// Returns the catalog URI.
    ///
    /// Returns
    /// -------
    /// str
    ///
    #[getter]
    fn catalog_uri(&self) -> String {
        self.inner.catalog_uri()
    }

    /// Returns the warehouse path.
    ///
    /// Returns
    /// -------
    /// str
    ///
    #[getter]
    fn warehouse_path(&self) -> String {
        self.inner.warehouse_path()
    }

    /// Returns the batch size.
    ///
    /// Returns
    /// -------
    /// int
    ///
    #[getter]
    fn batch_size(&self) -> usize {
        self.inner.batch_size
    }

    /// Returns the flush interval in seconds.
    ///
    /// Returns
    /// -------
    /// int
    ///
    #[getter]
    fn flush_interval_secs(&self) -> u64 {
        self.inner.flush_interval_secs
    }

    fn __repr__(&self) -> String {
        format!(
            "PyIcebergConfig(project_ref='{}', warehouse_name='{}', batch_size={}, flush_interval_secs={})",
            self.inner.project_ref,
            self.inner.warehouse_name,
            self.inner.batch_size,
            self.inner.flush_interval_secs
        )
    }
}

/// Python wrapper for `OrderbookL3Event`.
#[pyclass(module = "nautilus_trader.core.nautilus_pyo3.iceberg")]
#[derive(Clone)]
pub struct PyOrderbookL3Event {
    inner: OrderbookL3Event,
}

#[pymethods]
impl PyOrderbookL3Event {
    /// Creates a new L3 orderbook event.
    ///
    /// Parameters
    /// ----------
    /// timestamp : int
    ///     Event timestamp in microseconds since epoch.
    /// instrument_id : str
    ///     Trading instrument identifier (e.g., "BTC-PERP").
    /// exchange : str
    ///     Exchange name (e.g., "BINANCE").
    /// order_id : str
    ///     Unique order identifier.
    /// action : str
    ///     Order action: "ADD", "UPDATE", or "DELETE".
    /// side : str
    ///     Order side: "BUY" or "SELL".
    /// price : str
    ///     Order price (decimal string).
    /// quantity : str
    ///     Order quantity (decimal string).
    /// sequence_number : int
    ///     Monotonically increasing sequence number.
    ///
    /// Returns
    /// -------
    /// PyOrderbookL3Event
    ///
    /// Raises
    /// ------
    /// ValueError
    ///     If action or side values are invalid.
    ///
    #[new]
    #[allow(clippy::too_many_arguments)]
    fn new(
        timestamp: i64,
        instrument_id: String,
        exchange: String,
        order_id: String,
        action: String,
        side: String,
        price: String,
        quantity: String,
        sequence_number: u64,
    ) -> PyResult<Self> {
        let action_enum = OrderAction::from_str(&action)
            .ok_or_else(|| pyo3::exceptions::PyValueError::new_err(format!("Invalid action: {}", action)))?;

        let side_enum = OrderSide::from_str(&side)
            .ok_or_else(|| pyo3::exceptions::PyValueError::new_err(format!("Invalid side: {}", side)))?;

        Ok(Self {
            inner: OrderbookL3Event {
                timestamp,
                instrument_id,
                exchange,
                order_id,
                action: action_enum,
                side: side_enum,
                price,
                quantity,
                sequence_number,
            },
        })
    }

    fn __repr__(&self) -> String {
        format!(
            "PyOrderbookL3Event(instrument_id='{}', order_id='{}', action='{}', side='{}', price='{}', quantity='{}')",
            self.inner.instrument_id,
            self.inner.order_id,
            self.inner.action,
            self.inner.side,
            self.inner.price,
            self.inner.quantity
        )
    }
}

/// Python wrapper for `IcebergSink`.
#[pyclass(module = "nautilus_trader.core.nautilus_pyo3.iceberg")]
pub struct PyIcebergSink {
    inner: Option<IcebergSink>,
}

#[pymethods]
impl PyIcebergSink {
    /// Creates a new Iceberg sink.
    ///
    /// Parameters
    /// ----------
    /// config : PyIcebergConfig
    ///     The Iceberg configuration.
    ///
    /// Returns
    /// -------
    /// PyIcebergSink
    ///
    /// Raises
    /// ------
    /// RuntimeError
    ///     If the sink cannot be initialized.
    ///
    #[new]
    fn new(py: Python, config: PyIcebergConfig) -> PyResult<Self> {
        py.allow_threads(|| {
            pyo3_asyncio::tokio::get_runtime().block_on(async {
                let inner = IcebergSink::new(config.inner)
                    .await
                    .map_err(to_pyvalue_err)?;
                Ok(Self { inner: Some(inner) })
            })
        })
    }

    /// Creates a sink with custom table name and namespace.
    ///
    /// Parameters
    /// ----------
    /// config : PyIcebergConfig
    ///     The Iceberg configuration.
    /// namespace : str
    ///     The namespace (e.g., "default", "production").
    /// table_name : str
    ///     The table name.
    ///
    /// Returns
    /// -------
    /// PyIcebergSink
    ///
    /// Raises
    /// ------
    /// RuntimeError
    ///     If the sink cannot be initialized.
    ///
    #[staticmethod]
    fn with_table(
        py: Python,
        config: PyIcebergConfig,
        namespace: String,
        table_name: String,
    ) -> PyResult<Self> {
        py.allow_threads(|| {
            pyo3_asyncio::tokio::get_runtime().block_on(async {
                let inner = IcebergSink::with_table(config.inner, namespace, table_name)
                    .await
                    .map_err(to_pyvalue_err)?;
                Ok(Self { inner: Some(inner) })
            })
        })
    }

    /// Writes an event to the buffer.
    ///
    /// Parameters
    /// ----------
    /// event : PyOrderbookL3Event
    ///     The L3 orderbook event to write.
    ///
    /// Raises
    /// ------
    /// RuntimeError
    ///     If the sink is closed or the write fails.
    ///
    fn write_event(&mut self, event: PyOrderbookL3Event) -> PyResult<()> {
        let sink = self.inner.as_mut()
            .ok_or_else(|| pyo3::exceptions::PyRuntimeError::new_err("Sink is closed"))?;

        sink.write_event(event.inner).map_err(to_pyvalue_err)
    }

    /// Checks if the buffer should be flushed.
    ///
    /// Returns
    /// -------
    /// bool
    ///
    fn should_flush(&self) -> PyResult<bool> {
        let sink = self.inner.as_ref()
            .ok_or_else(|| pyo3::exceptions::PyRuntimeError::new_err("Sink is closed"))?;

        Ok(sink.should_flush())
    }

    /// Flushes buffered events to S3.
    ///
    /// Raises
    /// ------
    /// RuntimeError
    ///     If the flush operation fails.
    ///
    fn flush(&mut self, py: Python) -> PyResult<()> {
        let sink = self.inner.as_mut()
            .ok_or_else(|| pyo3::exceptions::PyRuntimeError::new_err("Sink is closed"))?;

        py.allow_threads(|| {
            pyo3_asyncio::tokio::get_runtime().block_on(async {
                sink.flush().await.map_err(to_pyvalue_err)
            })
        })
    }

    /// Returns the current number of buffered events.
    ///
    /// Returns
    /// -------
    /// int
    ///
    fn buffer_len(&self) -> PyResult<usize> {
        let sink = self.inner.as_ref()
            .ok_or_else(|| pyo3::exceptions::PyRuntimeError::new_err("Sink is closed"))?;

        Ok(sink.buffer_len())
    }

    /// Returns the table name.
    ///
    /// Returns
    /// -------
    /// str
    ///
    fn table_name(&self) -> PyResult<String> {
        let sink = self.inner.as_ref()
            .ok_or_else(|| pyo3::exceptions::PyRuntimeError::new_err("Sink is closed"))?;

        Ok(sink.table_name().to_string())
    }

    /// Closes the sink gracefully.
    ///
    /// Raises
    /// ------
    /// RuntimeError
    ///     If the close operation fails.
    ///
    fn close(&mut self, py: Python) -> PyResult<()> {
        let sink = self.inner.take()
            .ok_or_else(|| pyo3::exceptions::PyRuntimeError::new_err("Sink already closed"))?;

        py.allow_threads(|| {
            pyo3_asyncio::tokio::get_runtime().block_on(async {
                sink.close().await.map_err(to_pyvalue_err)
            })
        })
    }

    fn __repr__(&self) -> String {
        if let Some(sink) = &self.inner {
            format!(
                "PyIcebergSink(table_name='{}', namespace='{}', buffer_len={})",
                sink.table_name(),
                sink.namespace(),
                sink.buffer_len()
            )
        } else {
            "PyIcebergSink(closed)".to_string()
        }
    }
}
