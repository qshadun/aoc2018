use std::{collections::HashMap, collections::HashSet, fs::read_to_string};

fn main() {
    let input = read_to_string("inputs/input6.txt").unwrap();
    let coords: Vec<Coordinate> = input
        .lines()
        .map(|line| Coordinate::from_str(line))
        .collect();
    let mut grid = Grid::new(coords);
    grid.find_finite();
    part1(&grid);
    part2(&grid);
}

fn part1(grid: &Grid) {
    let mut ans = 0;
    for &loc in grid.finite.iter() {
        let mut area = 0;
        for &loc2 in grid.table.values() {
            if loc == loc2 {
                area += 1;
            }
        }
        ans = ans.max(area);
    }
    println!("{}", ans);
}

fn part2(grid: &Grid) {
    let bound = 500;
    let mut size = 0;
    for x in -bound..=bound {
        for y in -bound..=bound {
            if grid.distance_sum(Coordinate { x, y }) < 10000 {
                size += 1;
            }
        }
    }
    println!("{}", size);
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Coordinate {
    x: i32,
    y: i32,
}

impl Coordinate {
    fn from_str(s: &str) -> Self {
        let parts: Vec<_> = s.split(",").collect();
        let x: i32 = parts[0].parse().unwrap();
        let y: i32 = parts[1].trim().parse().unwrap();
        Self { x, y }
    }

    fn distance(self, other: Coordinate) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    fn border(self, step: i32) -> impl Iterator<Item = Coordinate> {
        (self.x - step..=self.x + step)
            .flat_map(move |x| (self.y - step..=self.y + step).map(move |y| Coordinate { x, y }))
            .filter(move |&c| self.distance(c) == step)
    }
}

#[derive(Debug)]
struct Grid {
    // all coordinates given in the input
    locations: Vec<Coordinate>,
    // all known finite coordinates
    finite: HashSet<Coordinate>,
    // a map from an arbitrary coordinate to its closest location
    table: HashMap<Coordinate, Coordinate>,
}

impl Grid {
    fn new(locations: Vec<Coordinate>) -> Self {
        Self {
            locations,
            finite: HashSet::new(),
            table: HashMap::new(),
        }
    }

    fn find_finite(&mut self) {
        // This isn't actually guaranteed to be correct. We simply assert that
        // after some fixed number of iterations, our set of finite locations
        // converges.
        //
        // I started this trying for a solution that didn't assume a bounding
        // box size, which would have made this much simpler. At the end of
        // the day, we're still not fully general because there is no logic
        // for detecting convergence.
        for step in 0..100 {
            for loc in self.locations.iter() {
                if self.finite.contains(loc) {
                    continue;
                }
                for c in loc.border(step) {
                    let closest = match self.closest_location(c) {
                        None => continue,
                        Some(closest) => closest,
                    };
                    self.table.insert(c, closest);
                }
            }
            for loc in self.locations.iter() {
                if loc.border(step).all(|c| self.table.get(&c) != Some(loc)) {
                    self.finite.insert(*loc);
                }
            }
        }
    }

    fn closest_location(&self, c: Coordinate) -> Option<Coordinate> {
        let mut min_cor = self.locations[0];
        let mut unique = true;
        for &loc in &self.locations[1..] {
            if loc.distance(c) == min_cor.distance(c) {
                unique = false;
            } else if loc.distance(c) < min_cor.distance(c) {
                unique = true;
                min_cor = loc;
            }
        }
        if unique {
            Some(min_cor)
        } else {
            None
        }
    }

    fn distance_sum(&self, c: Coordinate) -> i32 {
        self.locations.iter().map(|&loc| loc.distance(c)).sum()
    }
}
