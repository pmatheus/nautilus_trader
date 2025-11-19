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

from nautilus_trader.common.config import PositiveInt
from nautilus_trader.config import LiveDataClientConfig
from nautilus_trader.config import LiveExecClientConfig


class AsterDataClientConfig(LiveDataClientConfig, frozen=True):
    """
    Configuration for ``AsterDataClient`` instances.

    Parameters
    ----------
    base_url_http : str, optional
        The HTTP client custom endpoint override.
    base_url_ws : str, optional
        The WebSocket client custom endpoint override.
    http_proxy_url : str, optional
        Optional HTTP proxy URL.
    ws_proxy_url : str, optional
        Optional WebSocket proxy URL.
        Note: WebSocket proxy support is not yet implemented. This field is reserved
        for future functionality. Use `http_proxy_url` for REST API proxy support.
    http_timeout_secs : PositiveInt, default 10
        The timeout (seconds) for HTTP requests.

    """

    base_url_http: str | None = None
    base_url_ws: str | None = None
    http_proxy_url: str | None = None
    ws_proxy_url: str | None = None
    http_timeout_secs: PositiveInt = 10


class AsterExecClientConfig(LiveExecClientConfig, frozen=True):
    """
    Configuration for ``AsterExecutionClient`` instances.

    Parameters
    ----------
    private_key : str, optional
        The Aster EVM private key.
        If ``None`` then will source the `ASTER_PK` environment variable.
    user_address : str, optional
        The user address for sub-account trading.
        If ``None`` then will use the signer address (derived from private key).
    base_url_http : str, optional
        The HTTP client custom endpoint override.
    base_url_ws : str, optional
        The WebSocket client custom endpoint override.
    http_proxy_url : str, optional
        Optional HTTP proxy URL.
    ws_proxy_url : str, optional
        Optional WebSocket proxy URL.
        Note: WebSocket proxy support is not yet implemented. This field is reserved
        for future functionality. Use `http_proxy_url` for REST API proxy support.
    max_retries : PositiveInt, optional
        The maximum number of times a submit, cancel or modify order request will be retried.
    retry_delay_initial_ms : PositiveInt, optional
        The initial delay (milliseconds) between retries. Short delays with frequent retries may result in account bans.
    retry_delay_max_ms : PositiveInt, optional
        The maximum delay (milliseconds) between retries.
    http_timeout_secs : PositiveInt, default 10
        The timeout (seconds) for HTTP requests.

    Warnings
    --------
    A short `retry_delay` with frequent retries may result in account bans.

    """

    private_key: str | None = None
    user_address: str | None = None
    base_url_http: str | None = None
    base_url_ws: str | None = None
    http_proxy_url: str | None = None
    ws_proxy_url: str | None = None
    max_retries: PositiveInt | None = None
    retry_delay_initial_ms: PositiveInt | None = None
    retry_delay_max_ms: PositiveInt | None = None
    http_timeout_secs: PositiveInt = 10
