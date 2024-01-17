use std::fmt;
use std::fmt::Display;

use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq)]
enum ItemType {
    Armor,
    Weapon,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Item<'a> {
    name: &'a str,
    value: u32,
    kind: ItemType,
    cost: u32,
}

impl<'a> Display for Item<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}, {:?}: {}, cost: {}",
            self.name, self.kind, self.value, self.cost
        )
    }
}

const WEAPONS: [Item; 5] = [
    Item {
        name: "dagger",
        value: 4,
        cost: 8,
        kind: ItemType::Weapon,
    },
    Item {
        name: "shortsword",
        value: 5,
        cost: 10,
        kind: ItemType::Weapon,
    },
    Item {
        name: "warhammer",
        value: 6,
        cost: 25,
        kind: ItemType::Weapon,
    },
    Item {
        name: "longsword",
        value: 7,
        cost: 40,
        kind: ItemType::Weapon,
    },
    Item {
        name: "greataxe",
        value: 8,
        cost: 74,
        kind: ItemType::Weapon,
    },
];

const ARMOR: [Item; 5] = [
    Item {
        name: "leather",
        value: 1,
        cost: 13,
        kind: ItemType::Armor,
    },
    Item {
        name: "chainmail",
        value: 2,
        cost: 31,
        kind: ItemType::Armor,
    },
    Item {
        name: "splintmail",
        value: 3,
        cost: 53,
        kind: ItemType::Armor,
    },
    Item {
        name: "bandedmail",
        value: 4,
        cost: 75,
        kind: ItemType::Armor,
    },
    Item {
        name: "platemail",
        value: 5,
        cost: 102,
        kind: ItemType::Armor,
    },
];

const RINGS: [Item; 6] = [
    Item {
        name: "Damage +1",
        value: 1,
        cost: 25,
        kind: ItemType::Weapon,
    },
    Item {
        name: "Damage +2",
        value: 2,
        cost: 50,
        kind: ItemType::Weapon,
    },
    Item {
        name: "Damage +3",
        value: 3,
        cost: 100,
        kind: ItemType::Weapon,
    },
    Item {
        name: "Defense +1",
        value: 1,
        cost: 20,
        kind: ItemType::Armor,
    },
    Item {
        name: "Defense +2",
        value: 2,
        cost: 40,
        kind: ItemType::Armor,
    },
    Item {
        name: "Defense +3",
        value: 3,
        cost: 80,
        kind: ItemType::Armor,
    },
];

#[derive(Debug, Default, Clone, Copy)]
struct Gear<'a> {
    weapon: Option<Item<'a>>,
    armor: Option<Item<'a>>,
    ring1: Option<Item<'a>>,
    ring2: Option<Item<'a>>,
}

impl<'a> Gear<'a> {
    fn cost(&self) -> u32 {
        self.weapon.map(|x| x.cost).unwrap_or(0)
            + self.armor.map(|x| x.cost).unwrap_or(0)
            + self.ring1.map(|x| x.cost).unwrap_or(0)
            + self.ring2.map(|x| x.cost).unwrap_or(0)
    }

    fn damage(&self) -> u32 {
        let mut value = self.weapon.map(|item| item.value).unwrap_or(0);
        for ring in [self.ring1, self.ring2].into_iter().flatten() {
            value += match ring.kind {
                ItemType::Armor => 0,
                ItemType::Weapon => ring.value,
            };
        }
        value
    }

    fn armor(&self) -> u32 {
        let mut value = self.armor.map(|item| item.value).unwrap_or(0);
        for ring in [self.ring1, self.ring2].into_iter().flatten() {
            value += match ring.kind {
                ItemType::Armor => ring.value,
                ItemType::Weapon => 0,
            };
        }
        value
    }
}

struct Character<'a> {
    name: &'a str,
    hit_points: u32,
    gear: Gear<'a>,
}

impl<'a> Display for Character<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}, hit points: {}, armor: {}, damage: {}\n{:?}",
            self.name,
            self.hit_points,
            self.armor(),
            self.damage(),
            self.gear
        )
    }
}

impl<'a> Character<'a> {
    fn new(name: &'a str, hit_points: u32, gear: Gear<'a>) -> Self {
        Character { name, hit_points, gear }
    }

    fn armor(&self) -> u32 {
        self.gear.armor()
    }

    fn damage(&self) -> u32 {
        self.gear.damage()
    }
}

fn main() {
    let boss = Character::new(
        "boss",
        103,
        Gear {
            weapon: Some(WEAPONS[2]),
            armor: Some(ARMOR[1]),
            ring1: Some(RINGS[2]),
            ring2: None,
        },
    );
    let player = Character::new("player", 100, Default::default());

    println!("{}", boss);

    let mut results = Vec::new();

    let weapons: Vec<Option<Item>> = WEAPONS.into_iter().map(Some).chain([None, None]).collect();
    let armor: Vec<Option<Item>> = ARMOR.into_iter().map(Some).chain([None, None]).collect();
    let rings: Vec<Option<Item>> = RINGS.into_iter().map(Some).chain([None]).collect();

    let permutations = [weapons, armor, rings.clone(), rings].into_iter().multi_cartesian_product();

    for items in permutations {
        let (weapon, armor, ring1, ring2) = items.into_iter().collect_tuple().unwrap();
        if weapon.is_none() || ring1 == ring2 {
            continue;
        }

        let gear = Gear { weapon, armor, ring1, ring2 };
        let player_armor = gear.armor();
        let boss_damage = if player_armor < boss.gear.damage() {
            boss.gear.damage() - player_armor
        } else {
            1
        };
        let player_damage = if boss.armor() < gear.damage() {
            gear.damage() - boss.armor()
        } else {
            1
        };

        let rounds_until_player_loses =  (player.hit_points as f32 / boss_damage as f32).ceil();
        let rounds_until_boss_loses = (boss.hit_points as f32 / player_damage as f32).ceil();

        let win = rounds_until_boss_loses <= rounds_until_player_loses;
        let cost = gear.cost();
        results.push((win, (cost, gear)));
    }

    println!("Results\n-------");

    println!("Round 1");
    let mut win_by_cost: Vec<(u32, Gear)> = results
        .iter()
        .filter_map(|&(win, details)| if win { Some(details) } else { None })
        .collect();
    win_by_cost.sort_by_key(|(cost, _)| *cost);
    let (cost, items) = &win_by_cost[0];
    println!("{}, {:?}", cost, items);

    println!("Round 2");
    let mut lost_by_cost: Vec<(u32, Gear)> = results
        .iter()
        .filter_map(|&(win, details)| if !win { Some(details) } else { None })
        .collect();
    lost_by_cost.sort_by_key(|(cost, _)| *cost);
    let (cost, items) = &lost_by_cost.last().unwrap();
    println!("{}, {:?}", cost, items);
}
