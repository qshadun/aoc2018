use lazy_static::lazy_static;
use regex::Regex;
use std::cmp::max;
use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
    hash::Hash,
};

fn main() {
    let input = read_to_string("inputs/input23.txt").unwrap();
    part1(&input);
}

fn part1(input: &str) {
    let bots: Vec<Bot> = input.lines().map(|line| parse_bot(line)).collect();
    let mut max_idx = 0;
    for i in 1..bots.len() {
        if bots[i].r > bots[max_idx].r {
            max_idx = i;
        }
    }
    println!("max radius bot: {:?}", bots[max_idx]);

    let mut ans = 0;
    for bot in bots.iter() {
        if bots[max_idx].in_range(bot) {
            ans += 1;
        }
    }
    println!("part1={}", ans);
}

#[derive(Debug)]
struct Bot {
    cor: Coordinator,
    r: u64,
}

impl Bot {
    fn new(cor: Coordinator, r: u64) -> Self {
        Self { cor, r }
    }

    fn dist(&self, other: &Bot) -> u64 {
        self.cor.dist(&other.cor)
    }

    fn in_range(&self, other: &Bot) -> bool {
        self.dist(other) <= self.r
    }

    fn cor_in_range(&self, cor: &Coordinator) -> bool {
        self.cor.dist(cor) <= self.r
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Coordinator {
    x: i64,
    y: i64,
    z: i64,
}

impl Coordinator {
    fn new(x: i64, y: i64, z: i64) -> Self {
        Self { x, y, z }
    }

    fn dist(&self, other: &Coordinator) -> u64 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y) + self.z.abs_diff(other.z)
    }
}
fn parse_bot(line: &str) -> Bot {
    // pos=<9999306,44070497,46228534>, r=60389933
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"^pos=<(?P<x>[-0-9]+),(?P<y>[-0-9]+),(?P<z>[-0-9]+)>,\s*r=(?P<r>[0-9]+)")
                .unwrap();
    }
    let caps = RE.captures(line).unwrap();
    let x: i64 = caps["x"].parse().unwrap();
    let y: i64 = caps["y"].parse().unwrap();
    let z: i64 = caps["z"].parse().unwrap();
    let r: u64 = caps["r"].parse().unwrap();
    Bot::new(Coordinator::new(x, y, z), r)
}
