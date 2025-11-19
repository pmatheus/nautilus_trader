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
Coinbase live factories.
"""

from nautilus_trader.adapters.coinbase.config import CoinbaseDataClientConfig
from nautilus_trader.adapters.coinbase.config import CoinbaseExecClientConfig
from nautilus_trader.adapters.coinbase.data import CoinbaseDataClient
from nautilus_trader.adapters.coinbase.execution import CoinbaseExecutionClient
from nautilus_trader.cache.cache import Cache
from nautilus_trader.common.component import LiveClock
from nautilus_trader.live.factories import LiveDataClientFactory
from nautilus_trader.live.factories import LiveExecClientFactory
from nautilus_trader.msgbus.bus import MessageBus


class CoinbaseLiveDataClientFactory(LiveDataClientFactory):
    """
    Provides a factory for creating Coinbase data clients.
    """

    @staticmethod
    def create(
        msgbus: MessageBus,
        cache: Cache,
        clock: LiveClock,
        config: CoinbaseDataClientConfig,
    ) -> CoinbaseDataClient:
        """
        Create a new Coinbase data client.

        Parameters
        ----------
        msgbus : MessageBus
            The message bus for the client.
        cache : Cache
            The cache for the client.
        clock : LiveClock
            The clock for the client.
        config : CoinbaseDataClientConfig
            The configuration for the client.

        Returns
        -------
        CoinbaseDataClient

        """
        return CoinbaseDataClient(
            msgbus=msgbus,
            cache=cache,
            clock=clock,
            config=config,
        )


class CoinbaseLiveExecClientFactory(LiveExecClientFactory):
    """
    Provides a factory for creating Coinbase execution clients.
    """

    @staticmethod
    def create(
        msgbus: MessageBus,
        cache: Cache,
        clock: LiveClock,
        config: CoinbaseExecClientConfig,
    ) -> CoinbaseExecutionClient:
        """
        Create a new Coinbase execution client.

        Parameters
        ----------
        msgbus : MessageBus
            The message bus for the client.
        cache : Cache
            The cache for the client.
        clock : LiveClock
            The clock for the client.
        config : CoinbaseExecClientConfig
            The configuration for the client.

        Returns
        -------
        CoinbaseExecutionClient

        """
        return CoinbaseExecutionClient(
            msgbus=msgbus,
            cache=cache,
            clock=clock,
            config=config,
        )
