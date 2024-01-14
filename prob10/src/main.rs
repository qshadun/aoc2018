use std::fs::read_to_string;
use lazy_static::lazy_static;
use regex::Regex;

fn main() {
    let input = read_to_string("inputs/input10.txt").unwrap();
    let mut points: Vec<Point> = vec![];
    for line in input.lines() {
        points.push(Point::from_str(&line));
    }
    let mut board = Board::new(points);
    for i in 0..1000000 {
        
        if board.print() {
            println!("**** at time {} ****", i);
            break;
        }

        board.move_points();
    }

}

#[derive(Debug)]
struct Board {
    points: Vec<Point>,
}

impl Board {
    fn new(points: Vec<Point>) -> Self {
        Self{ points }
    }

    fn move_points(&mut self) {
        for p in &mut self.points {
            p.x += p.vx;
            p.y += p.vy;
        }
    }

    fn calc_bound(&self) -> Bound {
        let mut min_x = i32::MAX;
        let mut max_x: i32 = i32::MIN;
        let mut min_y: i32 = i32::MAX;
        let mut max_y: i32 = i32::MIN;
        for p in self.points.iter() {
            min_x = min_x.min(p.x);
            max_x = max_x.max(p.x);
            min_y = min_y.min(p.y);
            max_y = max_y.max(p.y);
        }
        Bound { min_x, max_x, min_y, max_y }
    }

    fn print(&self) -> bool {
        let bound = self.calc_bound();
        if bound.height() >= 11 {
            return false;
        }
        let mut grid = vec![vec!['.'; bound.width()]; bound.height()];
        for p in &self.points {
            let x = bound.normal_x(p.x);
            let y = bound.normal_y(p.y);
            grid[y][x] = '#';
        }
        for row in &grid {
            let row_str: String = row.iter().collect();
            println!("{}", row_str);
        }
        true
    }
}

#[derive(Clone, Copy, Debug, Default)]
struct Bound {
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
}

impl Bound {
    fn width(&self) -> usize {
        (self.max_x - self.min_x + 1) as usize
    }

    fn height(&self) -> usize {
        (self.max_y - self.min_y + 1) as usize 
    }

    fn normal_x(&self, x: i32) -> usize {
        (x - self.min_x) as usize
    }
    
    fn normal_y(&self, y: i32) -> usize {
        (y - self.min_y) as usize
    }
}

#[derive(Clone, Debug)]
struct Point {
    x: i32,
    y: i32,
    vx: i32,
    vy: i32,
}

impl Point {
    fn from_str(line: &str) -> Self {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(?x)
                position=<\s*(?P<x>[-0-9]+),\s*(?P<y>[-0-9]+)>
                \s+
                velocity=<\s*(?P<vx>[-0-9]+),\s*(?P<vy>[-0-9]+)>
            ").unwrap();
        }
        let caps = RE.captures(line).unwrap();
        Point {
            x: caps["x"].parse().unwrap(),
            y: caps["y"].parse().unwrap(),
            vx: caps["vx"].parse().unwrap(),
            vy: caps["vy"].parse().unwrap(),
        }
    }
}