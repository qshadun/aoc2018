use std::collections::{HashSet, VecDeque};
use std::fs::read_to_string;

fn main() {
    let input = read_to_string("inputs/input25.txt").unwrap();
    let points: HashSet<Point> = input.lines().map(|line| Point::new(line)).collect();
    let mut part1 = 0;

    let mut remain_points = points.clone();
    let mut q: VecDeque<Point> = VecDeque::new();
    while !remain_points.is_empty() {
        let p = remain_points.iter().next().unwrap().clone();
        remain_points.retain(|&x| x != p);
        q.push_back(p);
        while let Some(p) = q.pop_front() {
            let (connected, not_connected) = remain_points.iter().partition(|x| x.dist(&p) <= 3);
            for c in connected {
                q.push_back(c);
            }
            remain_points = not_connected;
        }
        part1 += 1;
    }

    println!("{}", part1);
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct Point {
    cords: [i32; 4],
}

impl Point {
    fn new(line: &str) -> Self {
        let parts: Vec<_> = line.split(',').collect();
        let mut cords = [0; 4];
        for i in 0..4 {
            cords[i] = parts[i].parse().unwrap();
        }
        Self { cords }
    }

    fn dist(&self, other: &Point) -> u32 {
        let mut ans = 0;
        for i in 0..4 {
            ans += self.cords[i].abs_diff(other.cords[i]);
        }
        ans
    }
}
