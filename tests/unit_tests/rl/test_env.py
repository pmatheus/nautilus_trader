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

from decimal import Decimal
from io import StringIO

from nautilus_trader.rl.env import ConsoleProgressSink
from nautilus_trader.rl.env import FailFastViolation
from nautilus_trader.rl.env import NautilusBacktestEnv


class StubBacktestSession:
    def __init__(self) -> None:
        self.actions: list[str] = []
        self.net_worths = [Decimal("100.00"), Decimal("101.25"), Decimal("99.75")]
        self.index = 0

    def reset(self, *, seed: int | None = None, options: dict | None = None):
        self.index = 0
        return {"bar": 0}, {"seed": seed, "options": options}

    def step(self, action: str):
        self.actions.append(action)
        self.index += 1
        return {"bar": self.index}, self.index == 2, False, {"action": action}

    def net_worth(self) -> Decimal:
        return self.net_worths[self.index]


def test_reset_returns_initial_observation_and_info() -> None:
    session = StubBacktestSession()
    env = NautilusBacktestEnv(session)

    observation, info = env.reset(seed=7, options={"start": "2026-01-01"})

    assert observation == {"bar": 0}
    assert info == {
        "seed": 7,
        "options": {"start": "2026-01-01"},
        "net_worth": Decimal("100.00"),
    }


def test_step_advances_backtest_and_rewards_delta_net_worth() -> None:
    session = StubBacktestSession()
    env = NautilusBacktestEnv(session)
    env.reset()

    observation, reward, terminated, truncated, info = env.step("BUY")

    assert session.actions == ["BUY"]
    assert observation == {"bar": 1}
    assert reward == Decimal("1.25")
    assert terminated is False
    assert truncated is False
    assert info == {
        "action": "BUY",
        "previous_net_worth": Decimal("100.00"),
        "net_worth": Decimal("101.25"),
    }


def test_reset_and_step_emit_progress_events() -> None:
    session = StubBacktestSession()
    emitted: list[dict] = []
    env = NautilusBacktestEnv(session, event_sink=emitted.append)

    env.reset(seed=7)
    env.step("BUY")

    assert emitted == [
        {
            "event": "reset",
            "step": 0,
            "net_worth": Decimal("100.00"),
            "episode_return": Decimal("0.00"),
            "return_pct": Decimal(0),
            "drawdown": Decimal(0),
            "seed": 7,
        },
        {
            "event": "step",
            "step": 1,
            "action": "BUY",
            "reward": Decimal("1.25"),
            "episode_return": Decimal("1.25"),
            "return_pct": Decimal("0.0125"),
            "drawdown": Decimal(0),
            "previous_net_worth": Decimal("100.00"),
            "net_worth": Decimal("101.25"),
            "terminated": False,
            "truncated": False,
        },
    ]
    assert env.events == emitted


def test_env_fails_fast_when_drawdown_exceeds_limit() -> None:
    session = StubBacktestSession()
    env = NautilusBacktestEnv(session, max_drawdown=Decimal("0.01"))
    env.reset()

    env.step("BUY")

    try:
        env.step("SELL")
    except FailFastViolation as exc:
        assert "drawdown" in str(exc)
        assert "0.01481481481481481481481481481" in str(exc)
    else:
        raise AssertionError("Expected FailFastViolation")


def test_console_progress_sink_formats_training_progress() -> None:
    stream = StringIO()
    sink = ConsoleProgressSink(stream=stream)

    sink(
        {
            "event": "step",
            "step": 3,
            "action": {"btc": "hold", "gold": "risk_on"},
            "reward": Decimal("12.50"),
            "episode_return": Decimal("42.00"),
            "return_pct": Decimal("0.0042"),
            "drawdown": Decimal("0.001"),
            "net_worth": Decimal("10042.00"),
            "terminated": False,
            "truncated": False,
        },
    )

    assert stream.getvalue() == (
        "[rl] step=3 reward=+12.50 net=10042.00 "
        "return=+0.42% dd=0.10% action={'btc': 'hold', 'gold': 'risk_on'}\n"
    )


def test_step_requires_reset_first() -> None:
    env = NautilusBacktestEnv(StubBacktestSession())

    try:
        env.step("HOLD")
    except RuntimeError as exc:
        assert str(exc) == "NautilusBacktestEnv.reset() must be called before step()."
    else:
        raise AssertionError("Expected RuntimeError")
