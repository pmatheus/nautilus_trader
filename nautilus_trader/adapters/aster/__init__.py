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
Aster DEX integration adapter.

This subpackage provides an instrument provider, data and execution clients,
configurations, and constants for connecting to and interacting with Aster DEX API.

For convenience, the most commonly used symbols are re-exported at the
subpackage's top level, so downstream code can simply import from
``nautilus_trader.adapters.aster``.
"""
from nautilus_trader.adapters.aster.config import AsterDataClientConfig
from nautilus_trader.adapters.aster.config import AsterExecClientConfig
from nautilus_trader.adapters.aster.constants import ASTER
from nautilus_trader.adapters.aster.constants import ASTER_CLIENT_ID
from nautilus_trader.adapters.aster.constants import ASTER_VENUE
from nautilus_trader.adapters.aster.factories import AsterLiveDataClientFactory
from nautilus_trader.adapters.aster.factories import AsterLiveExecClientFactory
from nautilus_trader.adapters.aster.providers import AsterInstrumentProvider


__all__ = [
    "ASTER",
    "ASTER_CLIENT_ID",
    "ASTER_VENUE",
    "AsterDataClientConfig",
    "AsterExecClientConfig",
    "AsterInstrumentProvider",
    "AsterLiveDataClientFactory",
    "AsterLiveExecClientFactory",
]
