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

from nautilus_trader.adapters.bitget.config import BitgetDataClientConfig
from nautilus_trader.adapters.bitget.config import BitgetExecClientConfig
from nautilus_trader.live.factories import LiveDataClientFactory
from nautilus_trader.live.factories import LiveExecClientFactory


class BitgetLiveDataClientFactory(LiveDataClientFactory):
    """
    Factory for creating Bitget live data clients.
    """

    @staticmethod
    def create(config: BitgetDataClientConfig):
        """
        Create a new Bitget live data client.

        Parameters
        ----------
        config : BitgetDataClientConfig
            The configuration for the client.

        Returns
        -------
        BitgetDataClient
            The created data client.

        """
        # TODO: Implement data client creation
        raise NotImplementedError("BitgetDataClient not yet fully implemented")


class BitgetLiveExecClientFactory(LiveExecClientFactory):
    """
    Factory for creating Bitget live execution clients.
    """

    @staticmethod
    def create(config: BitgetExecClientConfig):
        """
        Create a new Bitget live execution client.

        Parameters
        ----------
        config : BitgetExecClientConfig
            The configuration for the client.

        Returns
        -------
        BitgetExecutionClient
            The created execution client.

        """
        # TODO: Implement execution client creation
        raise NotImplementedError("BitgetExecutionClient not yet fully implemented")
