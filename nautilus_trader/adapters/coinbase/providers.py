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
Coinbase instrument provider.
"""

from nautilus_trader.common.providers import InstrumentProvider


class CoinbaseInstrumentProvider(InstrumentProvider):
    """
    Provides instrument definitions for the Coinbase exchange.

    Parameters
    ----------
    client : CoinbaseHttpClient
        The Coinbase HTTP client.
    """

    def __init__(self, client):
        """
        Initialize a new instance of the ``CoinbaseInstrumentProvider`` class.
        """
        super().__init__()
        self._client = client

    async def load_all_async(self, filters: dict | None = None) -> None:
        """
        Load all instruments for the venue asynchronously.

        Parameters
        ----------
        filters : dict, optional
            Not applicable for Coinbase.

        """
        # TODO: Implement instrument loading
        pass
