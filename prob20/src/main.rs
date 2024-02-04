use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

fn main() {
    let input = read_to_string("inputs/input20.txt").unwrap();
    let deltas = HashMap::from([
        ('W', (-1, 0)),
        ('E', (1, 0)),
        ('S', (0, -1)),
        ('N', (0, 1)),
    ]);

    let mut positions = vec![];
    let mut m: HashMap<(i32, i32), HashSet<(i32, i32)>> = HashMap::new();
    let mut distances: HashMap<(i32, i32), i32> = HashMap::new();
    let (mut x, mut y) = (5000, 5000);
    let (mut prev_x, mut prev_y) = (x, y);
    distances.insert((x, y), 0);
    for c in (&input[1..input.len()-1]).chars() {
        match c {
            '(' => positions.push((x, y)),
            ')' => (x, y) = positions.pop().unwrap(),
            '|' => (x, y) = *positions.last().unwrap(),
            _ => {
                let (dx, dy) = *deltas.get(&c).unwrap();
                x += dx;
                y += dy;
                m.entry((x, y)).or_default().insert((prev_x, prev_y));
                if !distances.contains_key(&(x, y)) {
                    distances.insert((x, y), distances.get(&(prev_x, prev_y)).unwrap() + 1);
                } else {
                    let cur = *distances.get(&(x, y)).unwrap();
                    let new_dist = distances.get(&(prev_x, prev_y)).unwrap() + 1;
                    if cur > new_dist {
                        distances.insert((x, y), new_dist);
                    }
                }
            }
        }
        (prev_x, prev_y) = (x, y);
    }

    let part1 = distances.values().max().unwrap();
    let part2 = distances.values().filter(|x| **x >=1000).count();
    println!("part1={}", part1);
    println!("part2={}", part2);
}




