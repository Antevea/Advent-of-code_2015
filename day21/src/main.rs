use std::fs::read_to_string;

#[derive(Clone, Copy, Debug)]
struct Item {
    cost: i32,
    damage: i32,
    armor: i32,
}

impl Item {
    fn new(cost: i32, damage: i32, armor: i32) -> Self {
        Self {
            cost,
            damage,
            armor
        }
    }
}

#[derive(Clone, Debug)]
struct Entity {
    armor: i32,
    damage: i32,
    hit_points: i32,
}

impl Entity {
    fn equip_items(&mut self, items: Vec<Item>) -> &mut Self {
        self.armor = items.iter().fold(0, |acc, itm| acc + itm.armor);
        self.damage = items.iter().fold(0, |acc, itm| acc + itm.damage);
        self
    }

    fn emulate_fight(&self, enemy: &Entity) -> bool {
        let mut boss = enemy.clone();
        let mut player = self.clone();
        let boss_effective_dmg = boss.damage - player.armor;
        let player_effective_dmg = player.damage - boss.armor;

        loop {
            boss.hit_points -= 1.max(player_effective_dmg);
            if boss.hit_points <= 0 {
                return true;
            }

            player.hit_points -= 1.max(boss_effective_dmg);
            if player.hit_points <= 0 {
                return false;
            }
        }
    }
}

fn solutions() {
    // let filepath = "sample";
    let filepath = "puzzle";
    let input_str = read_to_string(filepath).expect(&format!("Error: File {} not found!", filepath));

    let boss_stats_vec = input_str.lines().collect::<Vec<&str>>();
    let boss_hp = boss_stats_vec[0]
        .split(':')
        .last()
        .unwrap()
        .trim()
        .parse::<i32>()
        .unwrap();
    let boss_dmg = boss_stats_vec[1]
        .split(' ')
        .last()
        .unwrap()
        .trim()
        .parse::<i32>()
        .unwrap();
    let boss_armor = boss_stats_vec[2]
        .split(' ')
        .last()
        .unwrap()
        .trim()
        .parse::<i32>()
        .unwrap();
    let boss_entity = Entity {
        armor: boss_armor,
        damage: boss_dmg,
        hit_points: boss_hp,
    };

    let weapon_items = vec![
        Item::new(8,  4, 0),
        Item::new(10, 5, 0),
        Item::new(25, 6, 0),
        Item::new(40, 7, 0),
        Item::new(74, 8, 0),
    ];
    let armor_items = vec![
        Item::new(0, 0, 0),     // Player don't buy any armor
        Item::new(13, 0, 1),
        Item::new(31, 0, 2),
        Item::new(53, 0, 3),
        Item::new(75, 0, 4),
        Item::new(102,0, 5),
    ];
    let ring_items = vec![
        Item::new(0, 0, 0),     // Player don't buy one ring
        Item::new(0, 0, 0),     // Player don't buy one ring
        Item::new(25, 1, 0),
        Item::new(50, 2, 0),
        Item::new(100,3, 0),
        Item::new(20, 0, 1),
        Item::new(40, 0, 2),
        Item::new(80, 0, 3),
    ];
    let player_hp = 100;
    let mut min_cost = i32::MAX;
    let mut max_cost = 0i32;

    let mut player = Entity {
        armor: 0,
        damage: 0,
        hit_points: player_hp,
    };

    // All combinations
    for wpn in weapon_items.iter() {
        for armr in armor_items.iter() {
            for (i, first_ring) in ring_items.iter().enumerate() {
                for (j, second_ring) in ring_items.iter().enumerate() {
                    if i == j {
                        continue;
                    }
                    let equipment = vec![*wpn, *armr, *first_ring, *second_ring];
                    let equipment_cost = equipment.iter().fold(0, |acc, itm| acc + itm.cost);

                    player.equip_items(equipment);              // Remove this clone
                    if player.emulate_fight(&boss_entity) == true {
                        min_cost = min_cost.min(equipment_cost);
                    } else {
                        max_cost = max_cost.max(equipment_cost);
                    }
                }
            }
        }
    }

    println!("Part 1: {}", min_cost);
    println!("Part 2: {}", max_cost);
}

fn main() {
    solutions();
}
