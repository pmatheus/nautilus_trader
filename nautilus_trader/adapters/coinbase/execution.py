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
Coinbase execution client.
"""

from nautilus_trader.live.execution_client import LiveExecutionClient


class CoinbaseExecutionClient(LiveExecutionClient):
    """
    Provides an execution client for the Coinbase exchange.

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

    """

    def __init__(
        self,
        msgbus,
        cache,
        clock,
        config,
    ):
        """
        Initialize a new instance of the ``CoinbaseExecutionClient`` class.
        """
        super().__init__(
            msgbus=msgbus,
            cache=cache,
            clock=clock,
            config=config,
        )
        # TODO: Initialize HTTP and WebSocket clients

    async def _connect(self) -> None:
        """
        Connect the client.
        """
        # TODO: Implement connection logic
        pass

    async def _disconnect(self) -> None:
        """
        Disconnect the client.
        """
        # TODO: Implement disconnection logic
        pass
