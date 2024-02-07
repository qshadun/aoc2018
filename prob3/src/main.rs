use std::{collections::HashMap, fs::read_to_string};

#[derive(Debug)]
struct Rectangle {
    x1: i32, // left top x
    y1: i32, // left top y
    x2: i32, // right top x
    y2: i32, // right top y
    id: i32,
}

impl Rectangle {
    fn from_str(line: &str) -> Self {
        // #561 @ 529,973: 28x10
        let parts: Vec<&str> = line.split(" ").collect();
        let left_top: Vec<&str> = parts[2].split(",").collect();
        let x1 = left_top[0].parse::<i32>().unwrap();
        let y1 = left_top[1];
        let y1 = &y1[..y1.len() - 1];
        let y1 = y1.parse::<i32>().unwrap();
        let width_height: Vec<_> = parts[3].split("x").collect();
        let width = width_height[0].parse::<i32>().unwrap();
        let height = width_height[1].parse::<i32>().unwrap();
        let x2 = x1 + width;
        let y2 = y1 + height;
        let id = parts[0];
        let id = &id[1..];
        let id: i32 = id.parse().unwrap();
        Rectangle { x1, y1, x2, y2, id }
    }

    fn iter_points(&self) -> IterPoints {
        IterPoints {
            rec: self,
            px: self.x1,
            py: self.y1,
        }
    }
}

struct IterPoints<'c> {
    rec: &'c Rectangle,
    px: i32,
    py: i32,
}

impl<'c> Iterator for IterPoints<'c> {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<(i32, i32)> {
        if self.py >= self.rec.y2 {
            self.py = self.rec.y1;
            self.px += 1;
        }
        if self.px >= self.rec.x2 {
            None
        } else {
            let (px, py) = (self.px, self.py);
            self.py += 1;
            Some((px, py))
        }
    }
}

type Grid = HashMap<(i32, i32), i32>;

fn main() {
    let input = read_to_string("inputs/input3.txt").unwrap();
    let rectanges: Vec<Rectangle> = input
        .lines()
        .map(|line| Rectangle::from_str(line))
        .collect();
    let mut grid: Grid = Grid::new();
    for rec in rectanges.iter() {
        for (x, y) in rec.iter_points() {
            *grid.entry((x, y)).or_default() += 1;
        }
    }
    part1(&grid);
    part2(&rectanges, &grid);
}

fn part1(grid: &Grid) {
    let cnt = grid.values().filter(|c| **c > 1).count();
    println!("{}", cnt);
}

fn part2(rectangles: &[Rectangle], grid: &Grid) {
    for rec in rectangles {
        if rec.iter_points().all(|p| grid[&p] == 1) {
            println!("{}", rec.id);
            return;
        }
    }
}
