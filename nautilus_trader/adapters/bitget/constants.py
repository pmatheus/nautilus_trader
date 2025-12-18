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

from typing import Final

from nautilus_trader.model.identifiers import ClientId
from nautilus_trader.model.identifiers import Venue


BITGET: Final[str] = "BITGET"
BITGET_VENUE: Final[Venue] = Venue(BITGET)
BITGET_CLIENT_ID: Final[ClientId] = ClientId(BITGET)

# HTTP API base URLs
BITGET_SPOT_BASE_URL: Final[str] = "https://api.bitget.com"
BITGET_MIX_BASE_URL: Final[str] = "https://api.bitget.com"

# WebSocket URLs
BITGET_WS_PUBLIC_URL: Final[str] = "wss://ws.bitget.com/v2/ws/public"
BITGET_WS_PRIVATE_URL: Final[str] = "wss://ws.bitget.com/v2/ws/private"

# Testnet URLs
BITGET_TESTNET_HTTP_URL: Final[str] = "https://api.bitget.com"
BITGET_TESTNET_WS_PUBLIC_URL: Final[str] = "wss://ws.bitget.com/v2/ws/public"
BITGET_TESTNET_WS_PRIVATE_URL: Final[str] = "wss://ws.bitget.com/v2/ws/private"
