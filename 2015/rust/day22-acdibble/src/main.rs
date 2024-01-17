use std::collections::VecDeque;
use std::env;
use std::fs;
use std::path::Path;
use std::time::SystemTime;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Spell {
    MagicMissile,
    Drain,
    Shield,
    Poison,
    Recharge,
}

const SPELLBOOK: [Spell; 5] = [
    Spell::MagicMissile,
    Spell::Drain,
    Spell::Shield,
    Spell::Poison,
    Spell::Recharge,
];

#[derive(Clone, Copy, Debug)]
struct Person {
    hp: u32,
    damage: u32,
    armor: u32,
    mana: u32,
}

#[derive(Clone, Copy)]
struct Effect(Spell, u32);

impl Effect {
    fn new(spell: Spell) -> Option<Self> {
        match spell {
            Spell::Shield | Spell::Poison => Some(Self(spell, 6)),
            Spell::Recharge => Some(Self(spell, 5)),
            _ => None,
        }
    }

    fn run(&mut self, player: &mut Person, boss: &mut Person) {
        self.1 -= 1;
        match self.0 {
            Spell::Shield => {
                if self.1 == 0 {
                    player.armor -= 7;
                }
            }
            Spell::Poison => boss.hp = boss.hp.saturating_sub(3),
            Spell::Recharge => player.mana += 101,
            _ => unreachable!(),
        }
    }

    fn is_active(&self) -> bool {
        self.1 != 0
    }
}

impl Person {
    fn attack(&self, other: &mut Self) {
        let damage = match self.damage.saturating_sub(other.armor) {
            0 => 1,
            n => n,
        };
        other.hp = other.hp.saturating_sub(damage)
    }

    fn is_alive(&self) -> bool {
        self.hp > 0
    }

    fn cast(&mut self, spell: Spell, boss: &mut Person) -> Result<(Option<Effect>, u32), ()> {
        let cost: u32;

        match spell {
            Spell::MagicMissile => {
                cost = 53;
                boss.hp = boss.hp.saturating_sub(4);
            }
            Spell::Drain => {
                cost = 73;
                self.hp += 2;
                boss.hp = boss.hp.saturating_sub(2);
            }
            Spell::Shield => {
                cost = 113;
                self.armor += 7;
            }
            Spell::Poison => cost = 173,
            Spell::Recharge => cost = 229,
        }

        if self.mana < cost {
            return Err(());
        }

        self.mana -= cost;
        Ok((Effect::new(spell), cost))
    }
}

fn do_battle(boss: Person, hard_mode: bool) -> u32 {
    let mut minimum_mana = std::u32::MAX;

    let mut queue = VecDeque::new();

    for i in 0..SPELLBOOK.len() {
        queue.push_back((
            Person {
                hp: 50,
                armor: 0,
                damage: 0,
                mana: 500,
            },
            boss,
            i,
            Vec::<Effect>::new(),
            0,
        ))
    }

    while let Some(battle) = queue.pop_front() {
        let (mut player, mut boss, current_spell, mut effects, mut mana_expended) = battle;

        if hard_mode {
            player.hp -= 1;

            if !player.is_alive() {
                continue;
            }
        }

        for effect in &mut effects {
            effect.run(&mut player, &mut boss);
        }

        if !boss.is_alive() {
            minimum_mana = minimum_mana.min(mana_expended);
            continue;
        }

        effects.retain(|e| e.is_active());

        let spell = SPELLBOOK[current_spell];

        if effects.iter().any(|eff| eff.0 == spell) {
            continue;
        }

        mana_expended += match player.cast(spell, &mut boss) {
            Ok((Some(effect), cost)) => {
                effects.push(effect);
                cost
            }
            Ok((None, cost)) => cost,
            Err(()) => continue,
        };

        if mana_expended >= minimum_mana {
            continue;
        }

        if !boss.is_alive() {
            minimum_mana = minimum_mana.min(mana_expended);
            continue;
        }

        for effect in &mut effects {
            effect.run(&mut player, &mut boss);
        }

        if !boss.is_alive() {
            minimum_mana = minimum_mana.min(mana_expended);
            continue;
        }

        effects.retain(|e| e.is_active());

        boss.attack(&mut player);

        if !player.is_alive() {
            continue;
        }

        for i in 0..SPELLBOOK.len() {
            queue.push_back((player, boss, i, effects.clone(), mana_expended))
        }
    }

    minimum_mana
}

fn part_one(boss: Person) -> u32 {
    do_battle(boss, false)
}

fn part_two(boss: Person) -> u32 {
    do_battle(boss, true)
}

fn parse_boss(input: &str) -> Person {
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

    Person {
        hp,
        damage,
        armor: 0,
        mana: 0,
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

    let boss = parse_boss(&input);

    time_it(|| println!("part 1: {}", part_one(boss)));
    time_it(|| println!("part 2: {}", part_two(boss)));

    Ok(())
}
