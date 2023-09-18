from copy import deepcopy
from dataclasses import dataclass, field
from functools import partial
from itertools import count
from heapq import heappop, heappush
from pprint import pprint
from typing import Optional
import logging

logger = logging.getLogger(__name__)
logger.addHandler(logging.StreamHandler())


@dataclass
class Spell:
    name: str
    cost: int
    effect: bool = False
    turns: Optional[int] = None
    damage: int = 0
    heal: int = 0
    armor: int = 0
    mana: int = 0


spells = {
    "Magic Missile": Spell("Magic Missile", 53, damage=4),
    "Drain": Spell("Drain", 73, damage=2, heal=2),
    "Shield": Spell("Shield", 113, damage=0, armor=7, mana=0, effect=True, turns=6),
    "Poison": Spell("Poison", 173, damage=3, mana=0, effect=True, turns=6),
    "Recharge": Spell("Recharge", 229, mana=101, effect=True, turns=5),
}


@dataclass
class Player:
    hp: int
    damage: int = 0
    armor: int = 0
    mana: int = 0


@dataclass
class State:
    player: Player
    opponent: Player
    spell_cast: Spell
    active_spells: dict[str, Spell] = field(default_factory=dict)
    mana_spent: int = 0
    previous: Optional['State'] = None

    @property
    def history(self):
        return " => ".join(self.iter_path())

    def iter_path(self):
        next = self.previous

        while next is not None:
            yield next.spell_cast.name
            next = next.previous

        return


def resolve_active_spells(state):
    for name in list(state.active_spells.keys()):
        state.active_spells[name] -= 1
        spell = spells[name]

        if name == "Poison":
            state.opponent.hp -= spell.damage
        elif name == "Recharge":
            state.player.mana += spell.mana
        elif name == "Shield":
            if state.active_spells[name] == 0:
                state.player.armor -= spell.armor

        if state.active_spells[name] == 0:
            del state.active_spells[name]

    return state


def player_turn(state):
    spell = state.spell_cast

    if spell.cost > state.player.mana:
        return None
    elif spell.name in state.active_spells:
        return None

    if spell.effect:
        state.active_spells[spell.name] = spell.turns

        if spell.name == "Shield":
            state.player.armor += spell.armor
    else:
        state.opponent.hp -= spell.damage
        state.player.armor += spell.armor
        state.player.hp += spell.heal

    state.player.mana -= spell.cost
    state.mana_spent += spell.cost

    return state


def opponents_turn(state):
    state.player.hp -= max(state.opponent.damage - state.player.armor, 1)
    return state


def mode(hard, state):
    if hard:
        state.player.hp -= 1
    return state


@dataclass(order=True)
class QueueItem:
    priority: int
    tag: int
    state: State = field(compare=False)


def game_round(state, hard=False):
    for step in (
        partial(mode, hard),
        resolve_active_spells,
        player_turn,
        resolve_active_spells,
        opponents_turn,
    ):
        new_state = step(state)
        if new_state is None:
            return None
        elif new_state.player.hp <= 0:
            # lose
            return None
        elif new_state.opponent.hp <= 0:
            # win
            return new_state

        state = new_state

    return state


def run(hard=False):
    queue = []

    unique = count()

    for spell in spells.values():
        state = State(player=Player(hp=50, mana=500), opponent=Player(hp=55, damage=8), spell_cast=spell)
        heappush(
            queue, QueueItem(priority=0, tag=next(unique), state=state)
        )

    while queue:
        item = heappop(queue)

        new_state = game_round(item.state, hard=hard)
        if new_state is None:
            continue
        elif new_state.opponent.hp <= 0:
            return new_state
            break

        new_state.previous = item.state

        for spell in spells.values():
            next_state = deepcopy(new_state)
            next_state.spell_cast = spell
            heappush(
                queue,
                QueueItem(
                    priority=new_state.mana_spent,
                    tag=next(unique),
                    state=next_state,
                ),
            )

    return None


def main():
    import argparse

    parser = argparse.ArgumentParser(description='RPG')
    parser.add_argument('--debug', action='store_true')
    parser.add_argument('--verbose', action='store_true')
    parser.add_argument('--profile', action='store_true')

    args = parser.parse_args()
    if args.debug:
        logger.setLevel(logging.DEBUG)
        logger.debug("In debug")
    else:
        logger.setLevel(logging.INFO)

    # winning_state = None
    # if args.profile:
    #     import cProfile
    #     cProfile.run('state = run()', 'astar.stats')
    #     winning_state = state
    # else:
    #     winning_state = run()

    # print(winning_state.mana_spent)
    # if args.debug or args.verbose:
    #     logger.info("%s", winning_state.history)

    # if args.profile:
    #     import pstats
    #     from pstats import SortKey
    #     p = pstats.Stats('astar.stats')
    #     p.strip_dirs().sort_stats(SortKey.CUMULATIVE).print_stats()

    winning_state = run(hard=True)
    print(winning_state.mana_spent)
    if args.debug or args.verbose:
        logger.info("%s", winning_state.history)

main()
