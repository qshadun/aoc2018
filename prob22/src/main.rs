#![allow(non_snake_case)]
use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, HashMap};

fn main() {
    // depth: 11817
    // target: 9,751
    let modulo = 20183;
    let x_mul = 16807;
    let y_mul = 48271;
    let depth = 11817;
    let (X, Y): (usize, usize) = (9, 751);

    let matrix = calc_matrix(modulo, x_mul, y_mul, depth, X, Y, true);

    let mut part1 = 0;
    for y in 0..=Y {
        for x in 0..=X {
            part1 += matrix[y][x] % 3;
        }
    }
    println!("part1={}", part1);
    let ans2 = part2(matrix, X, Y);
    println!("part2={}", ans2);
}

fn part2(matrix: Vec<Vec<usize>>, X: usize, Y: usize) -> usize {
    let type_matrix: Vec<Vec<RegionType>> = matrix
        .iter()
        .map(|row| row.iter().map(|&v| RegionType::from_value(v)).collect())
        .collect();
    let mut heap = BinaryHeap::new();
    let start_position = Position::new(0, 0, Gear::Torch);
    let target_position = Position::new(X, Y, Gear::Torch);
    heap.push(Reverse(TimedPosition::new(0, start_position)));

    let mut min_time: HashMap<Position, usize> = HashMap::new();

    while let Some(Reverse(tp)) = heap.pop() {
        let TimedPosition { time, position } = tp;
        if position == target_position {
            return time;
        }
        match min_time.get(&position) {
            Some(t) if *t <= time => {
                continue;
            }
            _ => {}
        }
        min_time.insert(position, time);
        for new_pos in get_move_positions(&position, &type_matrix) {
            heap.push(Reverse(TimedPosition::new(time + 1, new_pos)));
        }
        let Position { x, y, gear } = position;
        heap.push(Reverse(TimedPosition::new(
            time + 7,
            Position::new(x, y, type_matrix[y][x].switch_gear(gear)),
        )));
    }
    0
}

fn get_move_positions(position: &Position, type_matrix: &Vec<Vec<RegionType>>) -> Vec<Position> {
    let Position { x, y, gear } = *position;
    let mut ans = vec![];
    if x > 0 && type_matrix[y][x - 1].allow_enter(gear) {
        ans.push(Position::new(x - 1, y, gear));
    }
    if x < type_matrix[0].len() - 1 && type_matrix[y][x + 1].allow_enter(gear) {
        ans.push(Position::new(x + 1, y, gear));
    }

    if y > 0 && type_matrix[y - 1][x].allow_enter(gear) {
        ans.push(Position::new(x, y - 1, gear));
    }
    if y < type_matrix.len() - 1 && type_matrix[y + 1][x].allow_enter(gear) {
        ans.push(Position::new(x, y + 1, gear));
    }
    ans
}

#[derive(Debug, Eq)]
struct TimedPosition {
    time: usize,
    position: Position,
}

impl TimedPosition {
    fn new(time: usize, position: Position) -> Self {
        Self { time, position }
    }
}

impl Ord for TimedPosition {
    fn cmp(&self, other: &Self) -> Ordering {
        self.time.cmp(&other.time)
    }
}

impl PartialOrd for TimedPosition {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for TimedPosition {
    fn eq(&self, other: &Self) -> bool {
        self.time == other.time
    }
}

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
struct Position {
    x: usize,
    y: usize,
    gear: Gear,
}

impl Position {
    fn new(x: usize, y: usize, gear: Gear) -> Position {
        Self { x, y, gear }
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
enum Gear {
    Torch,
    ClimbingGear,
    Neither,
}

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
enum RegionType {
    Rocky = 0,
    Wet = 1,
    Narrow = 2,
}

impl RegionType {
    fn from_value(i: usize) -> RegionType {
        match i {
            x if x == RegionType::Rocky as usize => RegionType::Rocky,
            x if x == RegionType::Wet as usize => RegionType::Wet,
            x if x == RegionType::Narrow as usize => RegionType::Narrow,
            _ => panic!("wrong region type"),
        }
    }

    fn allowed_gear(&self) -> (Gear, Gear) {
        match *self {
            RegionType::Rocky => (Gear::ClimbingGear, Gear::Torch),
            RegionType::Narrow => (Gear::Torch, Gear::Neither),
            RegionType::Wet => (Gear::ClimbingGear, Gear::Neither),
        }
    }

    fn switch_gear(&self, cur_gear: Gear) -> Gear {
        let (x, y) = self.allowed_gear();
        if x == cur_gear {
            y
        } else {
            x
        }
    }

    fn allow_enter(&self, cur_gear: Gear) -> bool {
        self.allowed_gear().0 == cur_gear || self.allowed_gear().1 == cur_gear
    }
}

fn calc_matrix(
    modulo: usize,
    x_mul: usize,
    y_mul: usize,
    depth: usize,
    X: usize,
    Y: usize,
    enlarge: bool,
) -> Vec<Vec<usize>> {
    let (ROW, COL) = if enlarge {
        let x = X.max(Y) * 2;
        (x, x)
    } else {
        (Y, X)
    };
    let mut erosion_matrix = vec![vec![0usize; COL + 1]; ROW + 1];
    erosion_matrix[0][0] = depth % modulo;
    erosion_matrix[Y][X] = erosion_matrix[0][0];

    for y in 0..=ROW {
        for x in 0..=COL {
            if x == 0 && y == 0 || x == X && y == Y {
                continue;
            }
            if x == 0 {
                erosion_matrix[y][x] = (y * y_mul + depth) % modulo;
            } else if y == 0 {
                erosion_matrix[y][x] = (x * x_mul + depth) % modulo;
            } else {
                erosion_matrix[y][x] =
                    (erosion_matrix[y - 1][x] * erosion_matrix[y][x - 1] + depth) % modulo;
            }
        }
    }

    let mut matrix = vec![vec![0usize; COL + 1]; ROW + 1];
    let mut part1 = 0;
    for y in 0..=ROW {
        for x in 0..=COL {
            matrix[y][x] = erosion_matrix[y][x] % 3;
        }
    }
    matrix
}
