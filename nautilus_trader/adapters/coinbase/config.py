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
"""
Coinbase configuration.
"""

from nautilus_trader.common.config import PositiveInt
from nautilus_trader.config import LiveDataClientConfig
from nautilus_trader.config import LiveExecClientConfig


class CoinbaseDataClientConfig(LiveDataClientConfig, frozen=True):
    """
    Configuration for ``CoinbaseDataClient`` instances.

    Parameters
    ----------
    api_key : str, optional
        The Coinbase API public key.
        If ``None`` then will source the `COINBASE_API_KEY` environment variable.
    api_secret : str, optional
        The Coinbase API secret key.
        If ``None`` then will source the `COINBASE_API_SECRET` environment variable.
    api_passphrase : str, optional
        The passphrase used when creating the Coinbase API keys.
        If ``None`` then will source the `COINBASE_PASSPHRASE` environment variable.
    base_url_http : str, optional
        The base URL for Coinbase HTTP API.
        If ``None`` then will use the default production URL.
    base_url_ws : str, optional
        The base URL for Coinbase WebSocket API.
        If ``None`` then will use the default production URL.
    is_sandbox : bool, default False
        If the client is connecting to the Coinbase sandbox API.
    update_instruments_interval_mins: PositiveInt, optional, default 60
        The interval (minutes) between reloading instruments from the venue.

    """

    api_key: str | None = None
    api_secret: str | None = None
    api_passphrase: str | None = None
    base_url_http: str | None = None
    base_url_ws: str | None = None
    is_sandbox: bool = False
    http_timeout_secs: PositiveInt | None = 60
    update_instruments_interval_mins: PositiveInt | None = 60


class CoinbaseExecClientConfig(LiveExecClientConfig, frozen=True):
    """
    Configuration for ``CoinbaseExecutionClient`` instances.

    Parameters
    ----------
    api_key : str, optional
        The Coinbase API public key.
        If ``None`` then will source the `COINBASE_API_KEY` environment variable.
    api_secret : str, optional
        The Coinbase API secret key.
        If ``None`` then will source the `COINBASE_API_SECRET` environment variable.
    api_passphrase : str, optional
        The passphrase used when creating the Coinbase API keys.
        If ``None`` then will source the `COINBASE_PASSPHRASE` environment variable.
    base_url_http : str, optional
        The base URL for Coinbase HTTP API.
        If ``None`` then will use the default production URL.
    base_url_ws : str, optional
        The base URL for Coinbase WebSocket API.
        If ``None`` then will use the default production URL.
    is_sandbox : bool, default False
        If the client is connecting to the Coinbase sandbox API.

    """

    api_key: str | None = None
    api_secret: str | None = None
    api_passphrase: str | None = None
    base_url_http: str | None = None
    base_url_ws: str | None = None
    is_sandbox: bool = False
    http_timeout_secs: PositiveInt | None = 60
