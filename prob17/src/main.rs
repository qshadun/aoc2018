use lazy_static::lazy_static;
use regex::Regex;
use std::fs::read_to_string;

fn main() {
    let input = read_to_string("inputs/input17.txt").unwrap();
    let mut game = Game::from_input(&input);
    game.print_board();
    game.flow()
}

#[derive(Debug)]
struct Game {
    board: Vec<Vec<char>>,
    min_x: i32,
}

impl Game {
    fn from_input(input: &str) -> Self {

        let (mut min_x, mut max_x, mut min_y, mut max_y) = (500, 500, i32::MAX, 0);
        let mut walls = vec![];
        for line in input.lines() {
            let xy = parse_line(line);
            if xy[0].len() == 1 {
                min_x = min_x.min(xy[0][0]);
                max_x = max_x.max(xy[0][0]);
                min_y = min_y.min(xy[1][0]);
                max_y = max_y.max(xy[1][1]);
            } else {
                min_x = min_x.min(xy[0][0]);
                max_x = max_x.max(xy[0][1]);
                min_y = min_y.min(xy[1][0]);
                max_y = max_y.max(xy[1][0]);
            }
            walls.push(xy);
        }
        min_x = min_x.min(500);
        max_x = max_x.max(500);
        min_x -= 1;
        max_x += 1;
        let mut board = vec![vec!['.'; (max_x - min_x + 1) as usize]; (max_y - min_y + 1) as usize];
        board[0][(500 - min_x) as usize] = '|';
        for xy in walls {
            if xy[0].len() == 1 {
                let x = (xy[0][0] - min_x) as usize;
                let y_lower = (xy[1][0] - min_y) as usize;
                let y_upper = (xy[1][1] - min_y) as usize;
                for y in y_lower..=y_upper {
                    board[y][x] = '#';
                }
            } else {
                let y = (xy[1][0] - min_y) as usize;
                let x_lower = (xy[0][0] - min_x) as usize;
                let x_upper = (xy[0][1] - min_x) as usize;
                for x in x_lower..=x_upper {
                    board[y][x] = '#';
                }
            }
        }
        Self {
            board,
            min_x,
        }
    }

    fn print_board(&self) {
        for row in self.board.iter() {
            let line: String = row.iter().collect();
            println!("{}", line);

        }
    }

    fn flow(&mut self) {
        let mut round = 0;
        let mut flowed = true;
        let num_cols = self.board[0].len();
        let num_rows = self.board.len();
        while flowed {
            flowed = false;
            for row in 0..num_rows-1 {
                for col in 0..num_cols{
                    if self.board[row][col] == '|' {
                        if self.board[row + 1][col] == '.' {
                            self.board[row + 1][col] = '|';
                            flowed = true
                        } else if is_floor(self.board[row + 1][col]) {
                            let mut left = col - 1;
                            while left >= 0 && self.board[row][left] != '#' && is_floor(self.board[row+1][left+1] ) {
                                if self.board[row][left] == '.' {
                                    self.board[row][left] = '|';
                                    flowed = true;
                                }
                                left -= 1;
                            }
                            let mut right = col + 1;
                            while right < num_cols && self.board[row][right] != '#' && is_floor(self.board[row+1][right-1] ) {
                                if self.board[row][right] == '.' {
                                    self.board[row][right] = '|';
                                    flowed = true;
                                }
                                right += 1;
                            }
                            if left >= 0 && self.board[row][left] == '#' && right < num_cols && self.board[row][right] == '#'
                                && is_floor(self.board[row+1][left+1]) && is_floor(self.board[row+1][right-1]) {
                                for col in left+1..right {
                                    self.board[row][col] = '~';
                                    flowed = true;
                                }
                            }
                        }
                    }
                }
            }
            round += 1;
            println!("========{}=======", round);
            // self.print_board();
        }

        let mut count = 0;
        let mut count2 = 0;
        for row in 0..num_rows {
            for col in 0..num_cols {
                if self.board[row][col] == '|' || self.board[row][col] == '~' {
                    count += 1;
                    if self.board[row][col] == '~' {
                        count2 += 1;
                    }
                }
            }
        }
        self.print_board();
        println!("{}", count);
        println!("{}", count2);
    }
}

fn is_floor(c: char) -> bool {
    c == '#' || c == '~'
}

fn parse_line(line: &str) -> Vec<Vec<i32>> {
    lazy_static! {
            static ref RE: Regex = Regex::new(r"(?P<a1>[x|y])=(?P<p1>[\d]+), (?P<a2>[x|y])=(?P<lower>[\d]+)\.\.(?P<upper>[\d]+)").unwrap();
        }
    let caps = RE.captures(line).unwrap();
    let a1 = &caps["a1"];
    let p1: i32 = caps["p1"].parse().unwrap();
    let lower: i32 = caps["lower"].parse().unwrap();
    let upper: i32 = caps["upper"].parse().unwrap();
    let mut ans = vec![];
    if a1 == "x" {
        ans.push(vec![p1]);
        ans.push(vec![lower, upper]);
    } else {
        ans.push(vec![lower, upper]);
        ans.push(vec![p1]);
    }
    ans
}
