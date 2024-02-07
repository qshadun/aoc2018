use std::fs::read_to_string;

// way to complicate, part2 is wrong
fn main() {
    let input = read_to_string("inputs/input20.txt").unwrap();
    let parts = parse(&input);
    println!("{:?}", parts);

    let e1 = "^WNE$";
    let e2 = "^ENWWW(NEEE|SSE(EE|N))$";
    let e3 = "^ENNWSWW(NEWS|)SSSEEN(WNSE|)EE(SWEN|)NNN$";
    let e4 = "^ESSWWN(E|NNENN(EESS(WNSE|)SSS|WWWSSSSE(SW|NNNE)))$";
    let e5 = "^WSSEESWWWNW(S|NENNEEEENN(ESSSSW(NWSW|SSEN)|WSWWN(E|WWS(E|SS))))$";
    println!("{:?}", parse(e1));
    println!("{:?}", parse(e2));
    println!("{:?}", parse(e3));
    println!("{:?}", parse(e4));
    println!("{:?}", parse(e5));

    part1(e1);
    part1(e2);
    part1(e3);
    part1(e4);
    part1(e5);
    part1(&input);
    part2(&input);
}

fn part1(regex: &str) {
    let mut maze = Maze::from_regex(regex);
    maze.calc_dist();
    println!("{}", maze.max_dist);
}

fn part2(regex: &str) {
    let mut maze = Maze::from_regex(regex);
    maze.calc_dist();
    println!("{}", maze.count);
}

// struct Node {
//     parent: Option<Weak<Node>>,
//     dist: usize,
//     length: usize,
//     children: Vec<Rc<Node>>
// }
//
// impl Node {
//
// }

struct Maze {
    parts: Vec<Part>,
    max_dist: usize,
    count: usize,
}

impl Maze {
    fn from_regex(regex: &str) -> Self {
        Self {
            parts: parse(regex),
            max_dist: 0,
            count: 0,
        }
    }

    fn calc_dist(&mut self) {
        self.calc_path(0, 0);
    }

    // return (length, next_idx)
    fn calc_path(&mut self, cur_dist: usize, start_idx: usize) -> (usize, usize) {
        let mut i = start_idx;
        let mut dist = 0;
        while i < self.parts.len() {
            match &self.parts[i] {
                Part::Path(s) => {
                    dist += s.len();
                    self.max_dist = self.max_dist.max(cur_dist + dist);
                    i += 1;
                    if cur_dist + dist >= 1000 {
                        self.count += s.len().min(cur_dist + dist - 999);
                    }
                }
                Part::CyclePath(s) => {
                    dist += s.len() / 2;
                    self.max_dist = self.max_dist.max(cur_dist + dist);
                    i += 1;
                    let half = s.len() / 2;
                    if cur_dist + dist >= 1000 {
                        self.count += half.min(cur_dist + dist - 999);
                    }
                }
                Part::BranchOpen => {
                    let (branch_len, next_idx) = self.calc_branch(cur_dist + dist, i + 1);
                    i = next_idx;
                    dist += branch_len;
                }
                Part::BranchSplit => {
                    return (dist, i + 1);
                }
                Part::BranchEnd => {
                    return (dist, i);
                }
            }
        }
        (dist, i)
    }

    fn calc_branch(&mut self, cur_dist: usize, start_idx: usize) -> (usize, usize) {
        let mut i = start_idx;
        let mut min_dist = usize::MAX;
        while i < self.parts.len() {
            match &self.parts[i] {
                Part::Path(_) => {
                    let (path_len, next_idx) = self.calc_path(cur_dist, i);
                    min_dist = min_dist.min(path_len);
                    i = next_idx;
                }
                Part::CyclePath(_) => {
                    let (_, next_idx) = self.calc_path(cur_dist, i);
                    min_dist = 0;
                    i = next_idx;
                }
                Part::BranchEnd => {
                    i += 1;
                    break;
                }
                _ => {
                    panic!("illegal path");
                }
            }
        }

        (min_dist, i)
    }
}
const DIRECTIONS: &str = "EWNS";
const OPEN_BRANCH: char = '(';
const CLOSE_BRANCH: char = ')';
const SPLIT: char = '|';

#[derive(Debug, Clone)]
enum Part {
    Path(String),
    CyclePath(String),
    BranchOpen,
    BranchEnd,
    BranchSplit,
}

fn parse(regex: &str) -> Vec<Part> {
    let chars: Vec<char> = (&regex[1..regex.len() - 1]).chars().collect();
    let mut parts = vec![];
    let mut i = 0;
    while i < chars.len() {
        if DIRECTIONS.contains(chars[i]) {
            let mut j = i + 1;
            while j < chars.len() && DIRECTIONS.contains(chars[j]) {
                j += 1;
            }
            let p: String = (&chars[i..j]).iter().collect();
            parts.push(Part::Path(p));
            i = j;
        } else {
            match chars[i] {
                OPEN_BRANCH => parts.push(Part::BranchOpen),
                CLOSE_BRANCH => parts.push(Part::BranchEnd),
                SPLIT => {
                    if chars[i + 1] == CLOSE_BRANCH {
                        // change to CyclePart
                        let mut cur_branches = vec![];
                        while let Some(p) = parts.pop() {
                            match p {
                                Part::Path(s) => cur_branches.push(Part::CyclePath(s)),
                                Part::BranchSplit => cur_branches.push(Part::BranchSplit),
                                Part::BranchOpen => {
                                    parts.push(Part::BranchOpen);
                                    break;
                                }
                                x => panic!("illegal parts {:?}", x),
                            }
                        }
                        while let Some(p) = cur_branches.pop() {
                            parts.push(p);
                        }
                    }
                    parts.push(Part::BranchSplit);
                }
                x => panic!("illega character {}", x),
            }
            i += 1;
        }
    }
    parts
}
