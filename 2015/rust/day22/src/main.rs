use std::cmp::{max, Ordering};
use std::collections::hash_map::Entry;
use std::collections::{BinaryHeap, HashMap};
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
const EMPTY_SPELL: Spell<'_> = Spell {
    name: "",
    cost: 0,
    damage: 0,
    armor: 0,
    heal: 0,
    mana: 0,
    duration: 0,
};

const SPELL_BOOK: [Spell; 5] = [
    Spell {
        name: "MagicMissile",
        cost: 53,
        damage: 4,
        ..EMPTY_SPELL
    },
    Spell {
        name: "Drain",
        cost: 73,
        damage: 2,
        heal: 2,
        ..EMPTY_SPELL
    },
    Spell {
        name: "Shield",
        cost: 113,
        armor: 7,
        duration: 6,
        ..EMPTY_SPELL
    },
    Spell {
        name: "Poison",
        cost: 173,
        damage: 3,
        duration: 6,
        ..EMPTY_SPELL
    },
    Spell {
        name: "Recharge",
        cost: 229,
        mana: 101,
        duration: 5,
        ..EMPTY_SPELL
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
    fn is_alive(&self) -> bool {
        self.hp > 0
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
            // The index helps prevent re-ordering.
            .then_with(|| other.index.cmp(&self.index))
    }
}

impl<'a> PartialOrd for State<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn pre_step(_player: &mut Player, _opponent: &mut Player, _gamestate: &mut GameState) -> GameFlow {
    GameFlow::Continue
}

fn hard_pre_step(
    player: &mut Player,
    _opponent: &mut Player,
    _gamestate: &mut GameState,
) -> GameFlow {
    player.hp = player.hp.saturating_sub(1);
    GameFlow::Continue
}

fn resolve_active_spells(
    player: &mut Player,
    opponent: &mut Player,
    gamestate: &mut GameState,
) -> GameFlow {
    if gamestate.active_spells.is_empty() {
        return GameFlow::Continue;
    }

    for (name, rounds_remaining) in gamestate.active_spells.iter_mut() {
        let spell = SPELL_BOOK.iter().find(|spell| spell.name == *name).unwrap();
        if spell.name == "Shield" && *rounds_remaining == spell.duration {
            player.armor += spell.armor;
        }

        *rounds_remaining = rounds_remaining.saturating_sub(1);

        opponent.hp -= spell.damage;
        player.mana += spell.mana;

        if *rounds_remaining == 0 && spell.name == "Shield" {
            player.armor -= spell.armor;
        }
    }

    gamestate
        .active_spells
        .retain(|_, rounds_remaining| *rounds_remaining > 0);
    GameFlow::Continue
}

fn player_turn(player: &mut Player, opponent: &mut Player, gamestate: &mut GameState) -> GameFlow {
    let spell = gamestate.spell_cast;

    if spell.cost > player.mana {
        return GameFlow::Invalid;
    }

    if spell.duration > 0 {
        if gamestate.active_spells.contains_key(spell.name) {
            return GameFlow::Invalid;
        }

        gamestate.active_spells.insert(spell.name, spell.duration);
    } else {
        opponent.hp = opponent.hp.saturating_sub(spell.damage);
        player.hp = player.hp.saturating_add(spell.heal);
        player.armor = player.armor.saturating_add(spell.armor);
    }

    player.mana = player.mana.saturating_sub(spell.cost);
    gamestate.mana_spent = gamestate.mana_spent.saturating_add(spell.cost);

    GameFlow::Continue
}

fn opponents_turn(
    player: &mut Player,
    opponent: &mut Player,
    _gamestate: &mut GameState,
) -> GameFlow {
    player.hp = player
        .hp
        .saturating_sub(max(opponent.damage.saturating_sub(player.armor), 1));
    GameFlow::Continue
}

fn game_round(
    hard_mode: bool,
    player: &mut Player,
    opponent: &mut Player,
    gamestate: &mut GameState,
) -> GameFlow {
    let steps = [
        if hard_mode { hard_pre_step } else { pre_step },
        resolve_active_spells,
        player_turn,
        resolve_active_spells,
        opponents_turn,
    ];

    for step in steps {
        let decision = step(player, opponent, gamestate);
        if decision != GameFlow::Continue {
            return decision;
        } else if !opponent.is_alive() {
            return GameFlow::Win;
        } else if !player.is_alive() {
            return GameFlow::Lose;
        }
    }

    GameFlow::Continue
}

fn check_if_best(
    previous_attempts: &mut HashMap<u32, (u32, u32, u32)>,
    player: &Player,
    opponent: &Player,
    gamestate: &GameState,
) -> GameFlow {
    match previous_attempts.entry(gamestate.mana_spent) {
        Entry::Occupied(mut occupied) => {
            let (best_opponent_hp, best_player_hp, best_mana) = occupied.get();
            // Less opponent hp is better.
            // More player hp is better.
            // More mana is better.
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
    let mut previous_attempts = HashMap::new();

    let mut unique_values = 0..;

    // First time this will fill in previous attempts with initial values.
    check_if_best(
        &mut previous_attempts,
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

    while let Some(State {
        mut player,
        mut opponent,
        mut gamestate,
        ..
    }) = queue.pop()
    {
        let decision = game_round(hard_mode, &mut player, &mut opponent, &mut gamestate);
        if decision == GameFlow::Win || !opponent.is_alive() {
            return Some(gamestate.mana_spent);
        } else if !player.is_alive() || decision != GameFlow::Continue {
            continue;
        }

        let decision = check_if_best(&mut previous_attempts, &player, &opponent, &gamestate);
        if decision != GameFlow::Continue {
            continue;
        }

        for spell in SPELL_BOOK {
            let new_gamestate = GameState {
                spell_cast: spell,
                ..gamestate.clone()
            };
            let next_state = State {
                player,
                opponent,
                gamestate: new_gamestate,
                index: unique_values.next().unwrap(),
            };
            queue.push(next_state);
        }
    }

    None
}

fn start<'a>(hard_mode: bool, initial_state: &'a State<'a>) -> Result<u32, &str> {
    match run(initial_state, hard_mode) {
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
        opponent,
        ..Default::default()
    };

    time_it(|| println!("part 1: {:?}", start(false, &initial_state)));
    time_it(|| println!("part 2: {:?}", start(true, &initial_state)));

    Ok(())
}
