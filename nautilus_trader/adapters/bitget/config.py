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

from nautilus_trader.common.config import PositiveInt
from nautilus_trader.config import LiveDataClientConfig
from nautilus_trader.config import LiveExecClientConfig

from nautilus_trader.adapters.bitget.types import BitgetInstrumentType


class BitgetDataClientConfig(LiveDataClientConfig, frozen=True):
    """
    Configuration for ``BitgetDataClient`` instances.

    Parameters
    ----------
    api_key : str, optional
        The Bitget API public key.
        If ``None`` then will source the `BITGET_API_KEY` environment variable.
    api_secret : str, optional
        The Bitget API secret key.
        If ``None`` then will source the `BITGET_API_SECRET` environment variable.
    api_passphrase : str, optional
        The passphrase used when creating the Bitget API keys.
        If ``None`` then will source the `BITGET_PASSPHRASE` environment variable.
    instrument_types : tuple[BitgetInstrumentType, ...], default (BitgetInstrumentType.SPOT,)
        The Bitget instrument types of instruments to load.
    base_url_http : str, optional
        The base URL for Bitget HTTP API.
    base_url_ws_public : str, optional
        The base URL for Bitget public WebSocket API.
    base_url_ws_private : str, optional
        The base URL for Bitget private WebSocket API.
    http_proxy_url : str, optional
        Optional HTTP proxy URL.
    ws_proxy_url : str, optional
        Optional WebSocket proxy URL.
    is_demo : bool, default False
        If the client is connecting to the Bitget demo/testnet API.
    http_timeout_secs : PositiveInt, optional
        The HTTP client timeout in seconds.
    max_retries : PositiveInt, optional
        The maximum retry attempts for requests.
    retry_delay_initial_ms : PositiveInt, optional
        The initial delay (milliseconds) between retries.
    retry_delay_max_ms : PositiveInt, optional
        The maximum delay (milliseconds) between retries.
    update_instruments_interval_mins : PositiveInt, optional
        The interval (minutes) between reloading instruments from the venue.

    """

    api_key: str | None = None
    api_secret: str | None = None
    api_passphrase: str | None = None
    instrument_types: tuple[BitgetInstrumentType, ...] = (BitgetInstrumentType.SPOT,)
    base_url_http: str | None = None
    base_url_ws_public: str | None = None
    base_url_ws_private: str | None = None
    http_proxy_url: str | None = None
    ws_proxy_url: str | None = None
    is_demo: bool = False
    http_timeout_secs: PositiveInt | None = 60
    max_retries: PositiveInt | None = 3
    retry_delay_initial_ms: PositiveInt | None = 1_000
    retry_delay_max_ms: PositiveInt | None = 10_000
    update_instruments_interval_mins: PositiveInt | None = 60


class BitgetExecClientConfig(LiveExecClientConfig, frozen=True):
    """
    Configuration for ``BitgetExecutionClient`` instances.

    Parameters
    ----------
    api_key : str, optional
        The Bitget API public key.
        If ``None`` then will source the `BITGET_API_KEY` environment variable.
    api_secret : str, optional
        The Bitget API secret key.
        If ``None`` then will source the `BITGET_API_SECRET` environment variable.
    api_passphrase : str, optional
        The passphrase used when creating the Bitget API keys.
        If ``None`` then will source the `BITGET_PASSPHRASE` environment variable.
    instrument_types : tuple[BitgetInstrumentType, ...], default (BitgetInstrumentType.SPOT,)
        The Bitget instrument types the execution client should support.
    base_url_http : str, optional
        The base URL for Bitget HTTP API.
    base_url_ws_private : str, optional
        The base URL for Bitget private WebSocket API.
    http_proxy_url : str, optional
        Optional HTTP proxy URL.
    ws_proxy_url : str, optional
        Optional WebSocket proxy URL.
    is_demo : bool, default False
        If the client is connecting to the Bitget demo/testnet API.
    http_timeout_secs : PositiveInt, optional
        The HTTP client timeout in seconds.
    max_retries : PositiveInt, optional
        The maximum retry attempts for requests.
    retry_delay_initial_ms : PositiveInt, optional
        The initial delay (milliseconds) between retries.
    retry_delay_max_ms : PositiveInt, optional
        The maximum delay (milliseconds) between retries.

    """

    api_key: str | None = None
    api_secret: str | None = None
    api_passphrase: str | None = None
    instrument_types: tuple[BitgetInstrumentType, ...] = (BitgetInstrumentType.SPOT,)
    base_url_http: str | None = None
    base_url_ws_private: str | None = None
    http_proxy_url: str | None = None
    ws_proxy_url: str | None = None
    is_demo: bool = False
    http_timeout_secs: PositiveInt | None = 60
    max_retries: PositiveInt | None = 3
    retry_delay_initial_ms: PositiveInt | None = 1_000
    retry_delay_max_ms: PositiveInt | None = 10_000
