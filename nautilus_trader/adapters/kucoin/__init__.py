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
Kucoin cryptocurrency exchange integration adapter.

This subpackage provides an instrument provider, data and execution clients,
configurations, and constants for connecting to and interacting with Kucoin's API.

UNIQUE FEATURE: Kucoin uses a token-based WebSocket connection system where
you must request a connection token via REST API before establishing WebSocket
connections. This is handled automatically by the adapter.
"""

from nautilus_trader.adapters.kucoin.config import KucoinDataClientConfig
from nautilus_trader.adapters.kucoin.config import KucoinExecClientConfig
from nautilus_trader.adapters.kucoin.constants import KUCOIN
from nautilus_trader.adapters.kucoin.constants import KUCOIN_CLIENT_ID
from nautilus_trader.adapters.kucoin.constants import KUCOIN_VENUE
from nautilus_trader.adapters.kucoin.factories import KucoinLiveDataClientFactory
from nautilus_trader.adapters.kucoin.factories import KucoinLiveExecClientFactory
from nautilus_trader.adapters.kucoin.providers import KucoinInstrumentProvider


__all__ = [
    "KUCOIN",
    "KUCOIN_CLIENT_ID",
    "KUCOIN_VENUE",
    "KucoinDataClientConfig",
    "KucoinExecClientConfig",
    "KucoinInstrumentProvider",
    "KucoinLiveDataClientFactory",
    "KucoinLiveExecClientFactory",
]
