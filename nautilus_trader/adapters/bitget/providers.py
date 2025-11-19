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

from decimal import Decimal

from nautilus_trader.adapters.bitget.constants import BITGET_VENUE
from nautilus_trader.adapters.bitget.types import BitgetInstrumentType
from nautilus_trader.common.providers import InstrumentProvider
from nautilus_trader.config import InstrumentProviderConfig
from nautilus_trader.core.correctness import PyCondition
from nautilus_trader.model.identifiers import InstrumentId
from nautilus_trader.model.identifiers import Symbol
from nautilus_trader.model.instruments import CryptoPerpetual
from nautilus_trader.model.instruments import CryptoFuture
from nautilus_trader.model.instruments import CurrencyPair
from nautilus_trader.model.objects import QUANTITY_MAX
from nautilus_trader.model.objects import QUANTITY_MIN
from nautilus_trader.model.objects import Currency
from nautilus_trader.model.objects import Money
from nautilus_trader.model.objects import Price
from nautilus_trader.model.objects import Quantity


class BitgetInstrumentProvider(InstrumentProvider):
    """
    Provides a means of loading `Instrument` objects for Bitget.

    Parameters
    ----------
    client : BitgetHttpClient
        The Bitget HTTP client (Rust).
    instrument_types : tuple[BitgetInstrumentType, ...]
        The instrument types to load.
    config : InstrumentProviderConfig, optional
        The instrument provider configuration.
    """

    def __init__(
        self,
        client,
        instrument_types: tuple[BitgetInstrumentType, ...],
        config: InstrumentProviderConfig | None = None,
    ):
        """Initialize the instrument provider."""
        super().__init__(config=config)
        self._client = client
        self._instrument_types = instrument_types
        self._log_warnings = config.log_warnings if config else True

    async def load_all_async(self, filters: dict | None = None) -> None:
        """
        Load all instruments for the configured instrument types.

        Parameters
        ----------
        filters : dict, optional
            Additional filters for loading instruments.
        """
        filters_str = "..." if not filters else f" with filters {filters}..."
        self._log.info(f"Loading all instruments{filters_str}")

        for inst_type in self._instrument_types:
            if inst_type == BitgetInstrumentType.SPOT:
                await self._load_spot_instruments()
            elif inst_type in (
                BitgetInstrumentType.USDT_FUTURES,
                BitgetInstrumentType.COIN_FUTURES,
                BitgetInstrumentType.USDC_FUTURES,
            ):
                await self._load_futures_instruments(inst_type)

    async def _load_spot_instruments(self) -> None:
        """Load spot instruments from Bitget."""
        try:
            instruments_data = await self._client.get_spot_instruments()

            for data in instruments_data:
                try:
                    instrument = self._parse_spot_instrument(data)
                    self.add(instrument=instrument)
                except Exception as e:
                    if self._log_warnings:
                        self._log.warning(f"Failed to parse spot instrument {data.get('symbol')}: {e}")
        except Exception as e:
            self._log.error(f"Failed to load spot instruments: {e}")

    async def _load_futures_instruments(self, inst_type: BitgetInstrumentType) -> None:
        """Load futures instruments from Bitget."""
        product_type = inst_type.value

        try:
            contracts_data = await self._client.get_futures_contracts(product_type)

            for data in contracts_data:
                try:
                    instrument = self._parse_futures_instrument(data, inst_type)
                    self.add(instrument=instrument)
                except Exception as e:
                    if self._log_warnings:
                        self._log.warning(
                            f"Failed to parse futures instrument {data.get('symbol')}: {e}",
                        )
        except Exception as e:
            self._log.error(f"Failed to load {product_type} instruments: {e}")

    def _parse_spot_instrument(self, data: dict) -> CurrencyPair:
        """Parse a spot instrument from API data."""
        symbol = data["symbol"]
        base_currency = Currency.from_str(data["baseCoin"])
        quote_currency = Currency.from_str(data["quoteCoin"])

        price_precision = data["priceScale"]
        size_precision = data["quantityScale"]
        price_increment = Price(10 ** -price_precision, precision=price_precision)
        size_increment = Quantity(10 ** -size_precision, precision=size_precision)

        min_quantity = Quantity(float(data["minTradeAmount"]), precision=size_precision)
        max_quantity = Quantity(float(data["maxTradeAmount"]), precision=size_precision)

        instrument_id = InstrumentId(
            symbol=Symbol(f"{symbol}-SPOT"),
            venue=BITGET_VENUE,
        )

        return CurrencyPair(
            instrument_id=instrument_id,
            raw_symbol=Symbol(symbol),
            base_currency=base_currency,
            quote_currency=quote_currency,
            price_precision=price_precision,
            size_precision=size_precision,
            price_increment=price_increment,
            size_increment=size_increment,
            min_quantity=min_quantity,
            max_quantity=max_quantity,
            min_notional=None,
            max_notional=None,
            min_price=None,
            max_price=None,
            margin_init=Decimal(0),
            margin_maint=Decimal(0),
            maker_fee=Decimal("0.001"),
            taker_fee=Decimal("0.001"),
            ts_event=0,
            ts_init=0,
        )

    def _parse_futures_instrument(
        self,
        data: dict,
        inst_type: BitgetInstrumentType,
    ) -> CryptoPerpetual:
        """Parse a futures instrument from API data."""
        symbol = data["symbol"]
        base_currency = Currency.from_str(data["baseCoin"])
        quote_currency = Currency.from_str(data["quoteCoin"])

        price_precision = data["pricePlace"]
        size_precision = data["volumePlace"]
        price_increment = Price.from_str(str(data["priceEndStep"]))
        size_increment = Quantity.from_str(str(data["volumeMultiplier"]))

        min_quantity = Quantity.from_str(str(data["minTradeNum"]))
        max_quantity = Quantity.from_str(str(data["maxTradeNum"]))

        # Determine settlement currency based on contract type
        if inst_type == BitgetInstrumentType.USDT_FUTURES:
            settlement_currency = Currency.from_str("USDT")
            suffix = "USDT-PERP"
        elif inst_type == BitgetInstrumentType.COIN_FUTURES:
            settlement_currency = base_currency
            suffix = "COIN-PERP"
        else:  # USDC
            settlement_currency = Currency.from_str("USDC")
            suffix = "USDC-PERP"

        instrument_id = InstrumentId(
            symbol=Symbol(f"{symbol}-{suffix}"),
            venue=BITGET_VENUE,
        )

        # Calculate margin based on leverage
        max_leverage = Decimal(str(data["maxLever"]))
        margin_init = Decimal(1) / max_leverage if max_leverage > 0 else Decimal("0.01")
        margin_maint = margin_init / Decimal(2)  # Approximation

        return CryptoPerpetual(
            instrument_id=instrument_id,
            raw_symbol=Symbol(symbol),
            base_currency=base_currency,
            quote_currency=quote_currency,
            settlement_currency=settlement_currency,
            is_inverse=inst_type == BitgetInstrumentType.COIN_FUTURES,
            price_precision=price_precision,
            size_precision=size_precision,
            price_increment=price_increment,
            size_increment=size_increment,
            max_quantity=max_quantity,
            min_quantity=min_quantity,
            max_notional=None,
            min_notional=None,
            max_price=None,
            min_price=None,
            margin_init=margin_init,
            margin_maint=margin_maint,
            maker_fee=Decimal("0.0002"),
            taker_fee=Decimal("0.0006"),
            ts_event=0,
            ts_init=0,
        )

    async def load_ids_async(
        self,
        instrument_ids: list[InstrumentId],
        filters: dict | None = None,
    ) -> None:
        """
        Load specific instruments by IDs.

        Parameters
        ----------
        instrument_ids : list[InstrumentId]
            The instrument IDs to load.
        filters : dict, optional
            Additional filters for loading instruments.
        """
        if not instrument_ids:
            self._log.warning("No instrument IDs given for loading")
            return

        # Validate all instrument IDs
        for instrument_id in instrument_ids:
            PyCondition.equal(
                instrument_id.venue,
                BITGET_VENUE,
                "instrument_id.venue",
                "BITGET",
            )

        # For now, load all and filter
        # TODO: Optimize to load only specific symbols
        await self.load_all_async(filters)

        # Filter to only requested IDs
        all_instruments = list(self.get_all().values())
        for instrument in all_instruments:
            if instrument.id not in instrument_ids:
                self.remove(instrument.id)

    async def load_async(
        self,
        instrument_id: InstrumentId,
        filters: dict | None = None,
    ) -> None:
        """
        Load a single instrument.

        Parameters
        ----------
        instrument_id : InstrumentId
            The instrument ID to load.
        filters : dict, optional
            Additional filters for loading the instrument.
        """
        PyCondition.not_none(instrument_id, "instrument_id")
        await self.load_ids_async([instrument_id], filters)
