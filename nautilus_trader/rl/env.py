# -------------------------------------------------------------------------------------------------
#  Copyright (C) 2015-2026 Nautech Systems Pty Ltd. All rights reserved.
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
Gym-like environment wrapper over a Nautilus backtest session.

The environment owns the reinforcement-learning contract while the wrapped
session owns deterministic market replay, execution, account state, and
portfolio valuation.
"""

from __future__ import annotations

from collections.abc import Callable
from decimal import Decimal
from typing import Any
from typing import Protocol
from typing import TextIO


type Observation = Any
type Action = Any
type Info = dict[str, Any]
type ProgressEvent = dict[str, Any]
type EventSink = Callable[[ProgressEvent], None]


class FailFastViolation(RuntimeError):
    """
    Raised when an RL episode violates a configured health boundary.
    """


class BacktestSession(Protocol):
    """
    Minimal adapter required by :class:`NautilusBacktestEnv`.

    Implementations are expected to be backed by Nautilus backtest components
    such as ``BacktestEngine`` or ``BacktestNode``.
    """

    def reset(
        self,
        *,
        seed: int | None = None,
        options: dict[str, Any] | None = None,
    ) -> tuple[Observation, Info]:
        """
        Reset the backtest episode and return the initial observation and info.
        """

    def step(self, action: Action) -> tuple[Observation, bool, bool, Info]:
        """
        Apply an agent action and advance the backtest to the next decision point.
        """

    def net_worth(self) -> Decimal:
        """
        Return the current account net worth after costs and valuation updates.
        """


class NautilusBacktestEnv:
    """
    Gym-like reinforcement-learning environment backed by a Nautilus session.

    The reward is the change in account net worth between decision points. This
    keeps fees, slippage, fills, and account accounting inside the Nautilus
    backtest kernel instead of reimplementing them in a separate simulator.
    """

    def __init__(
        self,
        session: BacktestSession,
        event_sink: EventSink | None = None,
        max_drawdown: Decimal | None = None,
    ) -> None:
        self._session = session
        self._event_sink = event_sink
        self._max_drawdown = max_drawdown
        self._events: list[ProgressEvent] = []
        self._initial_net_worth: Decimal | None = None
        self._peak_net_worth: Decimal | None = None
        self._previous_net_worth: Decimal | None = None
        self._step = 0

    @property
    def events(self) -> list[ProgressEvent]:
        """
        Return progress events emitted by the environment.
        """
        return list(self._events)

    @property
    def session(self) -> BacktestSession:
        """
        Return the wrapped Nautilus backtest session.
        """
        return self._session

    def reset(
        self,
        *,
        seed: int | None = None,
        options: dict[str, Any] | None = None,
    ) -> tuple[Observation, Info]:
        """
        Reset the episode and return ``(observation, info)``.
        """
        observation, info = self._session.reset(seed=seed, options=options)
        net_worth = self._session.net_worth()
        self._check_positive_net_worth(net_worth)
        self._initial_net_worth = net_worth
        self._peak_net_worth = net_worth
        self._previous_net_worth = net_worth
        self._step = 0
        self._emit(
            {
                "event": "reset",
                "step": self._step,
                "net_worth": net_worth,
                "episode_return": Decimal("0.00"),
                "return_pct": Decimal(0),
                "drawdown": Decimal(0),
                "seed": seed,
            },
        )

        return observation, {
            **info,
            "net_worth": net_worth,
        }

    def step(self, action: Action) -> tuple[Observation, Decimal, bool, bool, Info]:
        """
        Apply ``action`` and return ``(observation, reward, terminated, truncated, info)``.
        """
        if self._previous_net_worth is None:
            raise RuntimeError("NautilusBacktestEnv.reset() must be called before step().")

        previous_net_worth = self._previous_net_worth
        observation, terminated, truncated, info = self._session.step(action)
        net_worth = self._session.net_worth()
        self._check_positive_net_worth(net_worth)
        reward = net_worth - previous_net_worth
        episode_return = net_worth - self._initial_net_worth
        return_pct = episode_return / self._initial_net_worth
        if net_worth > self._peak_net_worth:
            self._peak_net_worth = net_worth
        drawdown = (self._peak_net_worth - net_worth) / self._peak_net_worth
        self._check_drawdown(drawdown)
        self._previous_net_worth = net_worth
        self._step += 1
        self._emit(
            {
                "event": "step",
                "step": self._step,
                "action": action,
                "reward": reward,
                "episode_return": episode_return,
                "return_pct": return_pct,
                "drawdown": drawdown,
                "previous_net_worth": previous_net_worth,
                "net_worth": net_worth,
                "terminated": terminated,
                "truncated": truncated,
            },
        )

        return observation, reward, terminated, truncated, {
            **info,
            "previous_net_worth": previous_net_worth,
            "net_worth": net_worth,
        }

    def _emit(self, event: ProgressEvent) -> None:
        self._events.append(event)
        if self._event_sink is not None:
            self._event_sink(event)

    def _check_positive_net_worth(self, net_worth: Decimal) -> None:
        if net_worth <= 0:
            raise FailFastViolation(f"net_worth must stay positive, got {net_worth}")

    def _check_drawdown(self, drawdown: Decimal) -> None:
        if self._max_drawdown is not None and drawdown > self._max_drawdown:
            raise FailFastViolation(
                f"drawdown {drawdown} exceeded max_drawdown {self._max_drawdown}",
            )


class ConsoleProgressSink:
    """
    Writes compact RL progress lines for humans and workflow logs.
    """

    def __init__(self, stream: TextIO) -> None:
        self._stream = stream

    def __call__(self, event: ProgressEvent) -> None:
        if event["event"] == "reset":
            self._stream.write(
                f"[rl] reset net={event['net_worth']} seed={event['seed']}\n",
            )
            return

        reward = event["reward"]
        return_pct = event["return_pct"] * Decimal(100)
        drawdown = event["drawdown"] * Decimal(100)
        self._stream.write(
            f"[rl] step={event['step']} "
            f"reward={reward:+} "
            f"net={event['net_worth']} "
            f"return={return_pct:+.2f}% "
            f"dd={drawdown:.2f}% "
            f"action={event['action']}\n",
        )
