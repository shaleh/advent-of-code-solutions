#![allow(dead_code)]

use std::cmp::{max, Ordering};
use std::collections::{BinaryHeap, HashMap};
use std::collections::hash_map::Entry;
use std::env;
use std::fs;
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
struct State<'a> {
    index: u32,
    mana_spent: u32,
    player: Player,
    opponent: Player,
    spell_cast: Spell<'a>,
    active_spells: HashMap<&'a str, u32>
}

impl<'a> Ord for State<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .mana_spent
            .cmp(&self.mana_spent)
            .then_with(|| other.index.cmp(&self.index))
    }
}

impl<'a> PartialOrd for State<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn run(opponent: &Player, hard_mode: bool) -> Option<u32> {
    let mut queue = BinaryHeap::new();
    let mut distances = HashMap::new();

    let mut unique = 0..;

    for spell in SPELL_BOOK {
        let state = State {
            player: Player {
                hp: 50,
                mana: 500,
                ..Default::default()
            },
            opponent: *opponent,
            spell_cast: spell,
            index: unique.next().unwrap(),
	    ..Default::default()
        };
        queue.push(state);
    }

    while let Some(mut state) = queue.pop() {
	let decision = /*'pre_step:*/ {
	    if hard_mode {
		state.player.hp = state.player.hp.saturating_sub(1);
	    }
	    GameFlow::Continue
	};
	if decision != GameFlow::Continue {
	    continue;
	} else if !state.opponent.is_alive() {
	    return Some(state.mana_spent);
	} else if !state.player.is_alive() {
	    continue;
	}

	let decision = 'resolve_active_spells_one: {
	    if state.active_spells.is_empty() {
		break 'resolve_active_spells_one GameFlow::Continue;
	    }

	    for (name, rounds_remaining) in state.active_spells.iter_mut() {
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

	    state.active_spells.retain(|_, rounds_remaining| *rounds_remaining > 0);
	    GameFlow::Continue
	};
	if decision != GameFlow::Continue {
	    continue;
	} else if !state.opponent.is_alive() {
	    return Some(state.mana_spent);
	} else if !state.player.is_alive() {
	    continue;
	}

	let decision = 'player_turn: {
	    if state.spell_cast.cost > state.player.mana {
		break 'player_turn GameFlow::Invalid;
	    }

	    if state.spell_cast.duration > 0 {
		if state.active_spells.contains_key(state.spell_cast.name) {
		    break 'player_turn GameFlow::Invalid;
		}

		state.active_spells.insert(state.spell_cast.name, state.spell_cast.duration);
	    } else {
		state.opponent.hp =  state.opponent.hp.saturating_sub( state.spell_cast.damage);
		state.player.hp =    state.player.hp.saturating_add(   state.spell_cast.heal);
		state.player.armor = state.player.armor.saturating_add(state.spell_cast.armor);
	    }

	    state.player.mana = state.player.mana.saturating_sub(state.spell_cast.cost);
	    state.mana_spent =  state.mana_spent.saturating_add( state.spell_cast.cost);

	    GameFlow::Continue
	};
	if decision != GameFlow::Continue {
	    continue;
	} else if !state.opponent.is_alive() {
	    return Some(state.mana_spent);
	} else if !state.player.is_alive() {
	    continue;
	}

	let decision = 'resolve_active_spells_two: {
	    if state.active_spells.is_empty() {
		break 'resolve_active_spells_two GameFlow::Continue;
	    }

	    for (name, rounds_remaining) in state.active_spells.iter_mut() {
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

	    state.active_spells.retain(|_, rounds_remaining| *rounds_remaining > 0);
	    GameFlow::Continue
	};
	if decision != GameFlow::Continue {
	    continue;
	} else if !state.opponent.is_alive() {
	    return Some(state.mana_spent);
	} else if !state.player.is_alive() {
	    continue;
	}

	let decision = /*'opponents_turn:*/ {
	    state.player.hp = state.player.hp.saturating_sub(max(state.opponent.damage.saturating_sub(state.player.armor), 1));
	    GameFlow::Continue
	};
	if decision != GameFlow::Continue {
	    continue;
	} else if !state.opponent.is_alive() {
	    return Some(state.mana_spent);
	} else if !state.player.is_alive() {
	    continue;
	}

	let decision = 'next_steps: {
	    match distances.entry(state.mana_spent) {
		Entry::Occupied(mut occupied) => {
		    let (best_opponent_hp, best_player_hp, best_mana) = occupied.get();
		    if state.opponent.hp > *best_opponent_hp
			|| state.player.hp < *best_player_hp
			|| state.player.mana < *best_mana
		    {
			// This iteration is not better than previous ones. Prune.
			break 'next_steps GameFlow::Invalid;
		    }
		    occupied.insert(
		        (
			    state.opponent.hp,
			    state.player.hp,
			    state.player.mana,
			)
		    );
		},
		Entry::Vacant(vacant) => {
		    vacant.insert(
			(
			    state.opponent.hp,
			    state.player.hp,
			    state.player.mana,
			)
		    );
		}
	    }

            for spell in SPELL_BOOK {
		let mut next_state = state.clone();
		next_state.index = unique.next().unwrap();
		next_state.spell_cast = spell;
		queue.push(next_state);
	    }

	    GameFlow::Continue
	};
	if decision != GameFlow::Continue {
	    continue;
	}
    }

    None
}

fn part_one(opponent: &Player) -> Result<u32, &str> {
    match run(opponent, false) {
        Some(mana_spent) => Ok(mana_spent),
        None => Err("No solution found"),
    }
}

fn part_two(opponent: &Player) -> Result<u32, &str> {
    match run(opponent, true) {
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

    time_it(|| println!("part 1: {:?}", part_one(&opponent)));
    time_it(|| println!("part 2: {:?}", part_two(&opponent)));

    Ok(())
}
