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
from nautilus_trader.core.nautilus_pyo3 import KucoinProductType


class KucoinDataClientConfig(LiveDataClientConfig, frozen=True):
    """
    Configuration for ``KucoinDataClient`` instances.

    Parameters
    ----------
    api_key : str, [default=None]
        The Kucoin API public key.
        If ``None`` then will source the `KUCOIN_API_KEY` environment variable.
    api_secret : str, [default=None]
        The Kucoin API secret key.
        If ``None`` then will source the `KUCOIN_API_SECRET` environment variable.
    api_passphrase : str, [default=None]
        The passphrase used when creating the Kucoin API keys.
        If ``None`` then will source the `KUCOIN_PASSPHRASE` environment variable.
    product_types : tuple[KucoinProductType, ...], optional
        The Kucoin product types for the client (SPOT, FUTURES).
        If not specified then will use SPOT only.
    base_url_http : str, optional
        The base url to Kucoin's http api.
    base_url_ws : str, optional
        The base url to Kucoin's websocket API (will be replaced with token-based URL).
    is_sandbox : bool, default False
        If the client is connecting to the Kucoin sandbox API.
    http_timeout_secs: PositiveInt or None, default 60
        The HTTP request timeout in seconds.
    max_retries: PositiveInt or None, default 3
        The maximum retry attempts for requests.

    """

    api_key: str | None = None
    api_secret: str | None = None
    api_passphrase: str | None = None
    product_types: tuple[KucoinProductType, ...] | None = None
    base_url_http: str | None = None
    base_url_ws: str | None = None
    is_sandbox: bool = False
    http_timeout_secs: PositiveInt | None = 60
    max_retries: PositiveInt | None = 3


class KucoinExecClientConfig(LiveExecClientConfig, frozen=True):
    """
    Configuration for ``KucoinExecutionClient`` instances.

    Parameters
    ----------
    api_key : str, [default=None]
        The Kucoin API public key.
        If ``None`` then will source the `KUCOIN_API_KEY` environment variable.
    api_secret : str, [default=None]
        The Kucoin API secret key.
        If ``None`` then will source the `KUCOIN_API_SECRET` environment variable.
    api_passphrase : str, [default=None]
        The passphrase used when creating the Kucoin API keys.
        If ``None`` then will source the `KUCOIN_PASSPHRASE` environment variable.
    base_url_http : str, optional
        The base url to Kucoin's http api.
    base_url_ws : str, optional
        The base url to Kucoin's websocket API (will be replaced with token-based URL).
    is_sandbox : bool, default False
        If the client is connecting to the Kucoin sandbox API.
    http_timeout_secs: PositiveInt or None, default 60
        The HTTP request timeout in seconds.
    max_retries: PositiveInt or None, default 3
        The maximum retry attempts for requests.

    """

    api_key: str | None = None
    api_secret: str | None = None
    api_passphrase: str | None = None
    base_url_http: str | None = None
    base_url_ws: str | None = None
    is_sandbox: bool = False
    http_timeout_secs: PositiveInt | None = 60
    max_retries: PositiveInt | None = 3
