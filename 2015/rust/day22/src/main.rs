#![allow(dead_code)]

use std::cmp::{max, Ordering};
use std::collections::hash_map::Entry;
use std::collections::{BinaryHeap, HashMap};
use std::env;
use std::fs;
use std::ops::RangeFrom;
use std::path::Path;
use std::time::SystemTime;

#[derive(Debug, PartialEq)]
enum GameFlow {
    Continue,
    Invalid,
    Lose,
    Win,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
struct Spell<'a> {
    name: &'a str,
    cost: u32,
    damage: u32,
    armor: u32,
    heal: u32,
    mana: u32,
    duration: u32,
}

const SPELL_BOOK: [Spell; 5] = [
    Spell {
        name: "MagicMissile",
        cost: 53,
        damage: 4,
        armor: 0,
        heal: 0,
        mana: 0,
        duration: 0,
    },
    Spell {
        name: "Drain",
        cost: 73,
        damage: 2,
        armor: 0,
        heal: 2,
        mana: 0,
        duration: 0,
    },
    Spell {
        name: "Shield",
        cost: 113,
        damage: 0,
        armor: 7,
        heal: 2,
        mana: 0,
        duration: 6,
    },
    Spell {
        name: "Poison",
        cost: 173,
        armor: 0,
        damage: 3,
        heal: 2,
        mana: 0,
        duration: 6,
    },
    Spell {
        name: "Recharge",
        cost: 229,
        armor: 0,
        damage: 0,
        heal: 0,
        mana: 101,
        duration: 5,
    },
];

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
struct Player {
    hp: u32,
    damage: u32,
    armor: u32,
    mana: u32,
}

impl Player {
    fn attack(&self, other: &mut Self) {
        let damage = match self.damage.saturating_sub(other.armor) {
            0 => 1,
            n => n,
        };
        other.hp = other.hp.saturating_sub(damage);
    }

    fn is_alive(&self) -> bool {
        self.hp > 0
    }

    fn cast(&mut self, spell: &Spell) -> Result<(), ()> {
        if self.mana < spell.cost {
            return Err(());
        }

        self.mana -= spell.cost;

        Ok(())
    }
}

#[derive(Clone, Default, Debug, Eq, PartialEq)]
struct GameState<'a> {
    mana_spent: u32,
    spell_cast: Spell<'a>,
    active_spells: HashMap<&'a str, u32>,
}

#[derive(Clone, Default, Debug, Eq, PartialEq)]
struct State<'a> {
    index: u32,
    player: Player,
    opponent: Player,
    gamestate: GameState<'a>,
}

impl<'a> Ord for State<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .gamestate
            .mana_spent
            .cmp(&self.gamestate.mana_spent)
            .then_with(|| other.index.cmp(&self.index))
    }
}

impl<'a> PartialOrd for State<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn pre_step(_state: &mut State) -> GameFlow {
    GameFlow::Continue
}

fn hard_pre_step(state: &mut State) -> GameFlow {
    state.player.hp = state.player.hp.saturating_sub(1);
    GameFlow::Continue
}

fn resolve_active_spells(state: &mut State) -> GameFlow {
    if state.gamestate.active_spells.is_empty() {
        return GameFlow::Continue;
    }

    for (name, rounds_remaining) in state.gamestate.active_spells.iter_mut() {
        let spell = SPELL_BOOK.iter().find(|spell| spell.name == *name).unwrap();
        if spell.name == "Shield" && *rounds_remaining == spell.duration {
            state.player.armor += spell.armor;
        }

        *rounds_remaining = rounds_remaining.saturating_sub(1);

        state.opponent.hp -= spell.damage;
        state.player.mana += spell.mana;

        if *rounds_remaining == 0 && spell.name == "Shield" {
            state.player.armor -= spell.armor;
        }
    }

    state
        .gamestate
        .active_spells
        .retain(|_, rounds_remaining| *rounds_remaining > 0);
    GameFlow::Continue
}

fn player_turn(state: &mut State) -> GameFlow {
    if state.gamestate.spell_cast.cost > state.player.mana {
        return GameFlow::Invalid;
    }

    if state.gamestate.spell_cast.duration > 0 {
        if state
            .gamestate
            .active_spells
            .contains_key(state.gamestate.spell_cast.name)
        {
            return GameFlow::Invalid;
        }

        state.gamestate.active_spells.insert(
            state.gamestate.spell_cast.name,
            state.gamestate.spell_cast.duration,
        );
    } else {
        state.opponent.hp = state
            .opponent
            .hp
            .saturating_sub(state.gamestate.spell_cast.damage);
        state.player.hp = state
            .player
            .hp
            .saturating_add(state.gamestate.spell_cast.heal);
        state.player.armor = state
            .player
            .armor
            .saturating_add(state.gamestate.spell_cast.armor);
    }

    state.player.mana = state
        .player
        .mana
        .saturating_sub(state.gamestate.spell_cast.cost);
    state.gamestate.mana_spent = state
        .gamestate
        .mana_spent
        .saturating_add(state.gamestate.spell_cast.cost);

    GameFlow::Continue
}

fn opponents_turn(state: &mut State) -> GameFlow {
    state.player.hp = state.player.hp.saturating_sub(max(
        state.opponent.damage.saturating_sub(state.player.armor),
        1,
    ));
    GameFlow::Continue
}

fn check_if_best(
    distances: &mut HashMap<u32, (u32, u32, u32)>,
    player: &Player,
    opponent: &Player,
    gamestate: &GameState,
) -> GameFlow {
    match distances.entry(gamestate.mana_spent) {
        Entry::Occupied(mut occupied) => {
            let (best_opponent_hp, best_player_hp, best_mana) = occupied.get();
            if opponent.hp > *best_opponent_hp
                || player.hp < *best_player_hp
                || player.mana < *best_mana
            {
                // This iteration is not better than previous ones. Prune.
                return GameFlow::Invalid;
            }
            occupied.insert((opponent.hp, player.hp, player.mana));
        }
        Entry::Vacant(vacant) => {
            vacant.insert((opponent.hp, player.hp, player.mana));
        }
    }

    GameFlow::Continue
}

fn run(initial_state: &State, hard_mode: bool) -> Option<u32> {
    let mut queue = BinaryHeap::new();
    let mut distances = HashMap::new();

    let mut unique_values = 0..;

    // First time this will fill in distances.
    check_if_best(
        &mut distances,
        &initial_state.player,
        &initial_state.opponent,
        &initial_state.gamestate,
    );

    for spell in SPELL_BOOK {
        let new_gamestate = GameState {
            spell_cast: spell,
            ..initial_state.gamestate.clone()
        };
        let next_state = State {
            player: initial_state.player,
            opponent: initial_state.opponent,
            gamestate: new_gamestate,
            index: unique_values.next().unwrap(),
        };
        queue.push(next_state);
    }

    let steps = [
	if hard_mode {
	    hard_pre_step
	} else {
	    pre_step
	},
	resolve_active_spells,
	player_turn,
	resolve_active_spells,
	opponents_turn,
    ];

    'outer: while let Some(mut state) = queue.pop() {
	for step in steps {
            let decision = step(&mut state);
	    if decision != GameFlow::Continue {
		continue 'outer;
	    } else if !state.opponent.is_alive() {
		return Some(state.gamestate.mana_spent);
	    } else if !state.player.is_alive() {
		continue 'outer;
	    }
	}

        let decision = check_if_best(
            &mut distances,
            &state.player,
            &state.opponent,
            &state.gamestate,
        );
        if decision != GameFlow::Continue {
            continue;
        }

        for spell in SPELL_BOOK {
            let new_gamestate = GameState {
                spell_cast: spell,
                ..state.gamestate.clone()
            };
            let next_state = State {
                player: state.player,
                opponent: state.opponent,
                gamestate: new_gamestate,
                index: unique_values.next().unwrap(),
            };
            queue.push(next_state);
        }
    }

    None
}

fn part_one<'a>(initial_state: &'a State<'a>) -> Result<u32, &str> {
    match run(initial_state, false) {
        Some(mana_spent) => Ok(mana_spent),
        None => Err("No solution found"),
    }
}

fn part_two<'a>(initial_state: &'a State<'a>) -> Result<u32, &str> {
    match run(initial_state, true) {
        Some(mana_spent) => Ok(mana_spent),
        None => Err("No solution found"),
    }
}

fn parse_opponent(input: &str) -> Player {
    let mut lines_it = input.lines();
    let line = lines_it.next().unwrap();
    let mut it = line.split_ascii_whitespace();
    it.next();
    it.next();
    let hp = it.next().unwrap().parse().unwrap();

    let line = lines_it.next().unwrap();
    let mut it = line.split_ascii_whitespace();
    it.next();
    let damage = it.next().unwrap().parse().unwrap();

    Player {
        hp,
        damage,
        ..Default::default()
    }
}

fn time_it<F>(fun: F)
where
    F: Fn(),
{
    let start = SystemTime::now();
    fun();
    println!("Time elapsed: {} ms", start.elapsed().unwrap().as_millis())
}

fn main() -> std::io::Result<()> {
    let file_path = env::current_dir()?.join(Path::new("data.txt"));
    let input = fs::read_to_string(file_path)?;

    let opponent = parse_opponent(&input);
    let initial_state = State {
        player: Player {
            hp: 50,
            mana: 500,
            ..Default::default()
        },
        opponent: opponent.clone(),
        ..Default::default()
    };

    time_it(|| println!("part 1: {:?}", part_one(&initial_state)));
    time_it(|| println!("part 2: {:?}", part_two(&initial_state)));

    Ok(())
}
