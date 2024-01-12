
use std::fs::read_to_string;

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

    fn contains(&self, x: i32, y: i32) -> bool {
        self.x1 <= x && self.x2 >= x && self.y1 <= y && self.y2 >= y
    }
}

fn main() {
    let input = read_to_string("input3").unwrap();
    // part1(&input);
    part2(&input);
}

fn part1(input: &str) {
    let mut max_x = 0;
    let mut max_y = 0;
    let mut ans = 0;
    let rectanges: Vec<Rectangle> = input
        .lines()
        .map(|line| Rectangle::from_str(line))
        .collect();
    for rec in rectanges.iter() {
        max_x = max_x.max(rec.x2);
        max_y = max_y.max(rec.y2);
    }
    
    for x in 1..=max_x {
        for y in 1..=max_y {
            let mut cnt: i32 = 0;
            for rec in rectanges.iter() {
                if rec.contains(x, y) && rec.contains(x - 1, y - 1) {
                    cnt += 1;
                    if cnt >= 2 {
                        ans += 1;
                        break;
                    }
                }
            }
        }
    }
    println!("{}", ans);
}

fn is_overlap(r1: &Rectangle, r2: &Rectangle) -> bool {
    !((r1.x2 <= r2.x1 || r1.y2 <= r2.y1) || (r2.x2 <= r1.x1 || r2.y2 <= r1.y1))
}

fn part2(input: &str) {
    let rectanges: Vec<Rectangle> = input
        .lines()
        .map(|line| Rectangle::from_str(line))
        .collect();
    let n = rectanges.len();
    for i in 0..n {
        let r1 = &rectanges[i];
        let mut is_intersect = false;
        for j in 0..n {
            if i == j {
                continue;
            }
            let r2 = &rectanges[j];
            if is_overlap(r1, r2) {
                is_intersect = true;
                break;
            }
        }
        if !is_intersect {
            println! {"{:?}", r1};
        }
    }
}
