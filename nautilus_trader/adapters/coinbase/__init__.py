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
Coinbase exchange (spot) integration adapter.

This subpackage provides an instrument provider, data and execution clients,
configurations, and constants for connecting to and interacting with Coinbase's API.
"""

from nautilus_trader.adapters.coinbase.config import CoinbaseDataClientConfig
from nautilus_trader.adapters.coinbase.config import CoinbaseExecClientConfig
from nautilus_trader.adapters.coinbase.constants import COINBASE
from nautilus_trader.adapters.coinbase.constants import COINBASE_CLIENT_ID
from nautilus_trader.adapters.coinbase.constants import COINBASE_VENUE
from nautilus_trader.adapters.coinbase.factories import CoinbaseLiveDataClientFactory
from nautilus_trader.adapters.coinbase.factories import CoinbaseLiveExecClientFactory
from nautilus_trader.adapters.coinbase.providers import CoinbaseInstrumentProvider


__all__ = [
    "COINBASE",
    "COINBASE_CLIENT_ID",
    "COINBASE_VENUE",
    "CoinbaseDataClientConfig",
    "CoinbaseExecClientConfig",
    "CoinbaseInstrumentProvider",
    "CoinbaseLiveDataClientFactory",
    "CoinbaseLiveExecClientFactory",
]
