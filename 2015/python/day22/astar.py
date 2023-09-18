from dataclasses import dataclass, field
from functools import partial
from itertools import count
from heapq import heappop, heappush
from pprint import pprint
import sys
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
    mana_spent: int = 0
    active_spells: dict[str, int] = field(default_factory=dict)
    previous: Optional['State'] = None

    @property
    def history(self):
        return " => ".join(spell.name for spell in self.iter_path())

    def iter_path(self):
        next = self.previous

        while next is not None:
            yield next.spell_cast
            next = next.previous

        return


def resolve_active_spells(state):
    for name, rounds_remaining in list(state.active_spells.items()):
        rounds_remaining -= 1
        spell = spells[name]

        state.opponent.hp -= spell.damage
        state.player.mana += spell.mana

        if rounds_remaining > 0:
            state.active_spells[name] -= 1
        else:
            if name == "Shield":
                state.player.armor -= spell.armor
            del state.active_spells[name]

    return state


def player_turn(state):
    spell = state.spell_cast

    if spell.cost > state.player.mana:
        logger.debug("too expensive to cast")
        return None

    if spell.effect:
        if spell.name in state.active_spells:
            logger.debug("already running")
            return None

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


def game_round(distances, state, hard=False):
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
            logger.debug("out of hp")
            return None
        elif new_state.opponent.hp <= 0:
            # win
            logger.debug("win")
            return new_state

        state = new_state

    return state


@dataclass(order=True)
class QueueItem:
    priority: int
    tag: int
    state: State = field(compare=False)


def run(hard=False):
    queue = []

    unique = count()

    distances = dict()

    for spell in spells.values():
        state = State(player=Player(hp=50, mana=500), opponent=Player(hp=55, damage=8), spell_cast=spell)
        heappush(
            queue, QueueItem(priority=0, tag=next(unique), state=state)
        )

    while queue:
        item = heappop(queue)

        new_state = game_round(distances, item.state, hard=hard)
        if new_state is None:
            continue
        elif new_state.opponent.hp <= 0:
            return new_state

        best_damage_dealt, best_player_hp, best_mana = distances.setdefault(new_state.mana_spent, (sys.maxsize, new_state.player.hp, new_state.player.mana))
        if new_state.opponent.hp > best_damage_dealt and new_state.player.hp < best_player_hp and new_state.player.mana < best_mana:
            logger.debug("%s, %s, %s, %s, %s, %s", new_state.spell_cast.name, new_state.mana_spent, new_state.history, new_state.opponent.hp, best_damage_dealt, best_mana)
            continue
            #pass

        distances[new_state.mana_spent] = (new_state.opponent.hp, new_state.player.hp, new_state.player.mana)

        for spell in spells.values():
            new_player = Player(hp=new_state.player.hp, mana=new_state.player.mana, armor=new_state.player.armor, damage=new_state.player.damage)
            new_opponent = Player(hp=new_state.opponent.hp, mana=new_state.opponent.mana, armor=new_state.opponent.armor, damage=new_state.opponent.damage)
            new_active_spells = dict(new_state.active_spells.items())

            heappush(
                queue,
                QueueItem(
                    priority=new_state.mana_spent,
                    tag=next(unique),
                    state=State(player=new_player, opponent=new_opponent, active_spells=new_active_spells, spell_cast=spell, mana_spent=new_state.mana_spent, previous=new_state),
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

    if args.profile:
        import cProfile
        cProfile.run('winning_state = run()', 'astar.stats')
    else:
        winning_state = run()

    print(winning_state.mana_spent)
    if args.debug or args.verbose:
        logger.info("%s", winning_state.history)

    if args.profile:
        import pstats
        from pstats import SortKey
        p = pstats.Stats('astar.stats')
        p.strip_dirs().sort_stats(SortKey.CUMULATIVE).print_stats()

    winning_state = run(hard=True)
    print(winning_state.mana_spent)
    if args.debug or args.verbose:
        logger.info("%s", winning_state.history)

main()
