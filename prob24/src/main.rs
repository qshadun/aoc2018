use crate::Side::{ImmuneSystem, Infection};
use lazy_static::lazy_static;
use regex::Regex;
use std::cmp::Reverse;
use std::fs::read_to_string;

fn main() {
    let input = read_to_string("inputs/input24.txt").unwrap();
    let game = Game::from_input(&input);
    let mut part1 = game.clone();
    part1.play();

    let mut boost = 1;
    loop {
        let mut part2 = game.clone();
        for g in part2.immune_system.iter_mut() {
            g.attack += boost;
        }
        println!("boost {}", boost);
        part2.play();
        if part2.infection.is_empty() {
            break;
        }
        boost += 1;
    }
}

#[derive(Debug, Clone)]
struct Group {
    units: i64,
    hit_point: i64,
    attack: i64,
    initiative: i64,
    attack_type: String,
    weakness: Vec<String>,
    immune: Vec<String>,
}

impl Group {
    fn parse_group(line: &str) -> Self {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"^(?P<units>\d+) units.* (?P<hit_point>\d+) hit.* (?P<attack>\d+) (?P<attack_type>\w+).* (?P<initiative>\d+)$"
            )
            .unwrap();
        }
        let caps = RE.captures(line).unwrap();
        let units: i64 = caps["units"].parse().unwrap();
        let hit_point: i64 = caps["hit_point"].parse().unwrap();
        let attack: i64 = caps["attack"].parse().unwrap();
        let attack_type: String = caps["attack_type"].parse().unwrap();
        let initiative: i64 = caps["initiative"].parse().unwrap();
        let (weakness, immune) = Self::get_weakness(line);
        Group {
            units,
            hit_point,
            attack,
            attack_type,
            initiative,
            weakness,
            immune,
        }
    }

    fn get_weakness(line: &str) -> (Vec<String>, Vec<String>) {
        let mut weakness = vec![];
        let mut immune = vec![];
        let start = line.find('(');
        if start.is_none() {
            return (weakness, immune);
        }
        let start = start.unwrap();
        let end = line.find(')').unwrap();
        let p = &line[start + 1..end];
        let parts: Vec<&str> = p.split("; ").collect();
        for p in parts {
            if p.starts_with("weak to ") {
                let w = &p["weak to ".len()..];
                let weaks: Vec<&str> = w.split(", ").collect();
                for weak in weaks {
                    weakness.push(weak.to_string());
                }
            } else {
                let w = &p["immune to ".len()..];
                let ims: Vec<&str> = w.split(", ").collect();
                for im in ims {
                    immune.push(im.to_string());
                }
            }
        }
        (weakness, immune)
    }

    fn calc_damage(&self, target: &Group) -> i64 {
        if target.immune.contains(&self.attack_type) {
            return 0;
        }
        if target.weakness.contains(&self.attack_type) {
            self.effective_power() * 2
        } else {
            self.effective_power()
        }
    }

    fn effective_power(&self) -> i64 {
        self.attack * self.units
    }
}

#[derive(Debug)]
enum Side {
    ImmuneSystem,
    Infection,
}

#[derive(Debug, Clone)]
struct Game {
    immune_system: Vec<Group>,
    infection: Vec<Group>,
}

impl Game {
    fn from_input(input: &str) -> Self {
        let mut immune_system = vec![];
        let mut infection = vec![];
        let mut cur_side = None;
        for line in input.lines() {
            if line.is_empty() {
                cur_side = None;
                continue;
            }
            if line == "Immune System:" {
                cur_side = Some("immune_system");
                continue;
            }
            if line == "Infection:" {
                cur_side = Some("infection");
                continue;
            }
            match cur_side {
                Some("immune_system") => {
                    immune_system.push(Group::parse_group(line));
                }
                Some("infection") => {
                    infection.push(Group::parse_group(line));
                }
                _ => {}
            }
        }
        Self {
            immune_system,
            infection,
        }
    }

    fn select_target(from: &Vec<Group>, to: &Vec<Group>) -> Vec<usize> {
        let mut ans = vec![to.len(); from.len()];
        let mut choose_order: Vec<usize> = (0..from.len()).collect();
        choose_order.sort_by_key(|&i| Reverse((from[i].effective_power(), from[i].initiative)));
        for from_idx in choose_order {
            let cur_group = &from[from_idx];
            let mut candidates: Vec<usize> = (0..to.len()).collect();
            candidates = candidates
                .into_iter()
                .filter(|i| !ans.contains(i))
                .collect();
            if candidates.is_empty() {
                continue;
            }
            candidates.sort_by_key(|&i| {
                (
                    cur_group.calc_damage(&to[i]),
                    to[i].effective_power(),
                    to[i].initiative,
                )
            });
            let chosen_idx = candidates[candidates.len() - 1];
            if cur_group.calc_damage(&to[chosen_idx]) != 0 {
                ans[from_idx] = chosen_idx;
            }
        }
        ans
    }

    fn attack(&mut self, immune_target: Vec<usize>, infect_target: Vec<usize>) -> bool {
        let mut unit_dead = false;
        let attack_order = self.attack_order();
        for (side, idx) in attack_order {
            let (cur_group, target) = match side {
                ImmuneSystem => {
                    let cur_group = &mut self.immune_system[idx];
                    if cur_group.units <= 0 {
                        continue; // no unites
                    }
                    let target_idx = immune_target[idx];
                    if target_idx == self.infection.len() {
                        continue; // no target
                    }
                    (cur_group, &mut self.infection[target_idx])
                }
                Infection => {
                    let cur_group = &mut self.infection[idx];
                    if cur_group.units <= 0 {
                        continue;
                    }
                    let target_idx = infect_target[idx];
                    if target_idx == self.immune_system.len() {
                        continue;
                    }
                    (cur_group, &mut self.immune_system[target_idx])
                }
            };
            let damage = cur_group.calc_damage(target);
            let unit_killed = damage / target.hit_point;
            if unit_killed > 0 {
                target.units -= unit_killed;
                unit_dead = true;
            }
        }
        self.immune_system = self
            .immune_system
            .iter()
            .filter(|g| g.units > 0)
            .map(|g| g.clone())
            .collect();
        self.infection = self
            .infection
            .iter()
            .filter(|g| g.units > 0)
            .map(|g| g.clone())
            .collect();
        unit_dead
    }

    fn attack_order(&self) -> Vec<(Side, usize)> {
        let mut attack_order = vec![];
        for i in 0..self.immune_system.len() {
            attack_order.push((ImmuneSystem, i));
        }
        for i in 0..self.infection.len() {
            attack_order.push((Infection, i));
        }
        attack_order.sort_by_key(|(side, idx)| match side {
            ImmuneSystem => Reverse(self.immune_system[*idx].initiative),
            Infection => Reverse(self.infection[*idx].initiative),
        });
        attack_order
    }

    fn play(&mut self) {
        while !self.immune_system.is_empty() && !self.infection.is_empty() {
            let immune_target = Self::select_target(&self.immune_system, &self.infection);
            let infection_target = Self::select_target(&self.infection, &self.immune_system);
            if !self.attack(immune_target, infection_target) {
                println!("tied");
                break;
            }
        }
        if self.immune_system.is_empty() {
            println!("infection wins");
            let ans: i64 = self.infection.iter().map(|g| g.units).sum();
            println!("remain units sum: {}", ans);
        } else if self.infection.is_empty() {
            println!("immune system wins");
            let ans: i64 = self.immune_system.iter().map(|g| g.units).sum();
            println!("remain units sum: {}", ans);
        }
    }
}
