# -------------------------------------------------------------------------------------------------
#  Copyright (C) 2015-2025 Nautech Systems Pty Ltd. All rights reserved.
#  https://nautechsystems.io
#
#  Licensed under the GNU Lesser General Public License Version 3.0 (the "License");
#  You may not use this file except in compliance with the License.
#  You may obtain a copy of the License at https://www.gnu.org/licenses/lgpl-3.0.en.html
#
#  Unless required by applicable law or agreed to in writing, software
#  distributed under the License is distributed on an "AS IS" BASIS,
#  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
#  See the License for the specific language governing permissions and
#  limitations under the License.
# -------------------------------------------------------------------------------------------------

from __future__ import annotations

from nautilus_trader.adapters.bitget.enums import BitgetMarginMode
from nautilus_trader.adapters.bitget.enums import BitgetPositionMode
from nautilus_trader.adapters.bitget.enums import BitgetProductType
from nautilus_trader.config import LiveDataClientConfig
from nautilus_trader.config import LiveExecClientConfig
from nautilus_trader.config import PositiveFloat
from nautilus_trader.config import PositiveInt


class BitgetDataClientConfig(LiveDataClientConfig, frozen=True):
    """
    Configuration for ``BitgetDataClient`` instances.

    Parameters
    ----------
    api_key : str, optional
        The Bitget API public key.
        If ``None`` then will source the `BITGET_API_KEY` or
        `BITGET_TESTNET_API_KEY` environment variables.
    api_secret : str, optional
        The Bitget API secret key.
        If ``None`` then will source the `BITGET_API_SECRET` or
        `BITGET_TESTNET_API_SECRET` environment variables.
    passphrase : str, optional
        The Bitget API passphrase.
        If ``None`` then will source the `BITGET_PASSPHRASE` or
        `BITGET_TESTNET_PASSPHRASE` environment variables.
    product_types : tuple[BitgetProductType, ...], optional
        The Bitget product types for the client.
        If not specified then will use all products.
    base_url_http : str, optional
        The base URL for Bitget HTTP API.
        If ``None`` then will use the default URL based on environment.
    base_url_ws_public : str, optional
        The base URL for Bitget public WebSocket API.
        If ``None`` then will use the default URL based on environment.
    base_url_ws_private : str, optional
        The base URL for Bitget private WebSocket API.
        If ``None`` then will use the default URL based on environment.
    http_proxy_url : str, optional
        Optional HTTP proxy URL.
    ws_proxy_url : str, optional
        Optional WebSocket proxy URL.
        Note: WebSocket proxy support is not yet implemented. This field is reserved
        for future functionality. Use `http_proxy_url` for REST API proxy support.
    testnet : bool, default False
        If the client is connecting to the Bitget testnet API.
    update_instruments_interval_mins: PositiveInt or None, default 60
        The interval (minutes) between reloading instruments from the venue.
    max_retries : PositiveInt, optional
        The maximum number of times an HTTP request will be retried.
    retry_delay_initial_ms : PositiveInt, optional
        The initial delay (milliseconds) between retries.
    retry_delay_max_ms : PositiveInt, optional
        The maximum delay (milliseconds) between retries.
    recv_window_ms : PositiveInt, default 5000
        The receive window (milliseconds) for Bitget HTTP requests.
    bars_timestamp_on_close : bool, default True
        If the ts_event timestamp for bars should be on the open or close of the bar.
        If True, then ts_event will be on the close of the bar.

    """

    api_key: str | None = None
    api_secret: str | None = None
    passphrase: str | None = None
    product_types: tuple[BitgetProductType, ...] | None = None
    base_url_http: str | None = None
    base_url_ws_public: str | None = None
    base_url_ws_private: str | None = None
    http_proxy_url: str | None = None
    ws_proxy_url: str | None = None
    testnet: bool = False
    update_instruments_interval_mins: PositiveInt | None = 60
    max_retries: PositiveInt | None = None
    retry_delay_initial_ms: PositiveInt | None = None
    retry_delay_max_ms: PositiveInt | None = None
    recv_window_ms: PositiveInt = 5_000
    bars_timestamp_on_close: bool = True


class BitgetExecClientConfig(LiveExecClientConfig, frozen=True):
    """
    Configuration for ``BitgetExecutionClient`` instances.

    Parameters
    ----------
    api_key : str, optional
        The Bitget API public key.
        If ``None`` then will source the `BITGET_API_KEY` or
        `BITGET_TESTNET_API_KEY` environment variables.
    api_secret : str, optional
        The Bitget API secret key.
        If ``None`` then will source the `BITGET_API_SECRET` or
        `BITGET_TESTNET_API_SECRET` environment variables.
    passphrase : str, optional
        The Bitget API passphrase.
        If ``None`` then will source the `BITGET_PASSPHRASE` or
        `BITGET_TESTNET_PASSPHRASE` environment variables.
    product_types : tuple[BitgetProductType, ...], optional
        The Bitget product types for the client.
        If None then will default to 'SPOT', you also cannot mix 'SPOT' with
        any other product type for execution.
    base_url_http : str, optional
        The base URL for Bitget HTTP API.
        If ``None`` then will use the default URL based on environment.
    base_url_ws_public : str, optional
        The base URL for Bitget public WebSocket API.
        If ``None`` then will use the default URL based on environment.
    base_url_ws_private : str, optional
        The base URL for Bitget private WebSocket API.
        If ``None`` then will use the default URL based on environment.
    http_proxy_url : str, optional
        Optional HTTP proxy URL.
    ws_proxy_url : str, optional
        Optional WebSocket proxy URL.
        Note: WebSocket proxy support is not yet implemented. This field is reserved
        for future functionality. Use `http_proxy_url` for REST API proxy support.
    testnet : bool, default False
        If the client is connecting to the Bitget testnet API.
    use_gtd : bool, default False
        If False, then GTD time in force will be remapped to GTC
        (this is useful if managing GTD orders locally).
    ignore_uncached_instrument_executions : bool, default False
        If True, execution messages for instruments not contained in the cache are ignored instead of raising an error.
    max_retries : PositiveInt, optional
        The maximum number of times a submit, cancel or modify order request will be retried.
    retry_delay_initial_ms : PositiveInt, optional
        The initial delay (milliseconds) between retries. Short delays with frequent retries may result in account bans.
    retry_delay_max_ms : PositiveInt, optional
        The maximum delay (milliseconds) between retries.
    recv_window_ms : PositiveInt, default 5000
        The receive window (milliseconds) for Bitget HTTP requests.
    ws_timeout_secs : PositiveFloat, default 5.0
        The timeout for WebSocket messages.
    futures_leverages : dict[str, PositiveInt], optional
        The leverages for futures symbols.
    position_mode : BitgetPositionMode, optional
        The position mode for futures products.
    margin_mode : BitgetMarginMode, optional
        The margin mode for futures products.

    Warnings
    --------
    A short `retry_delay` with frequent retries may result in account bans.

    """

    api_key: str | None = None
    api_secret: str | None = None
    passphrase: str | None = None
    product_types: tuple[BitgetProductType, ...] | None = None
    base_url_http: str | None = None
    base_url_ws_public: str | None = None
    base_url_ws_private: str | None = None
    http_proxy_url: str | None = None
    ws_proxy_url: str | None = None
    testnet: bool = False
    use_gtd: bool = False
    ignore_uncached_instrument_executions: bool = False
    max_retries: PositiveInt | None = None
    retry_delay_initial_ms: PositiveInt | None = None
    retry_delay_max_ms: PositiveInt | None = None
    recv_window_ms: PositiveInt = 5_000
    ws_timeout_secs: PositiveFloat = 5.0
    futures_leverages: dict[str, PositiveInt] | None = None
    position_mode: BitgetPositionMode | None = None
    margin_mode: BitgetMarginMode | None = None
