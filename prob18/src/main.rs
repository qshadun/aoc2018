use std::collections::HashMap;
use std::fs::read_to_string;

fn main() {
    let input = read_to_string("inputs/input18.txt").unwrap();
    let mut game = Game::new(&input);
    game.print_matrix();
    game.play(10000);
    println!("{}", game.scores[10]);
    for (&score, turns) in game.seen.iter() {
        if turns.len() > 1 {
            let period = turns[1] - turns[0];
            let delta = (1000000000 - turns[0]) % period;
            println!("{}", game.scores[turns[0] + delta]);
        }
    }


}

#[derive(Debug)]
struct Game {
    matrix: Vec<Vec<char>>,
    turn: usize,
    scores: Vec<usize>,
    seen: HashMap<usize, Vec<usize>>,
}


impl Game {
    fn new(input: &str) -> Self {
        let mut matrix = vec![];
        for line in input.lines() {
            let row: Vec<char> = line.chars().collect();
            matrix.push(row);
        }
        let initial_score = calc_score(&matrix);
        let mut seen = HashMap::new();
        seen.insert(initial_score, vec![0]);
        Self {
            matrix,
            turn: 0,
            scores: vec![initial_score],
            seen: seen,
        }
    }

    fn print_matrix(&self) {
        for row in self.matrix.iter() {
            let s: String = row.iter().collect();
            println!("{}", s);
        }
    }

    fn play(&mut self, turn: usize) {
        for i in 1..=turn {
            self.one_turn();
            self.turn += 1;
            // println!("===== turn {} =====", self.turn);
            let score = self.get_score();
            self.scores.push(score);
            self.seen.entry(score).or_insert(vec![]).push(self.turn);
        }
    }
    fn one_turn(&mut self) {
        let mut new_matrix = vec![vec!['.'; self.matrix[0].len()]; self.matrix.len()];
        for r in 0..self.matrix.len() {
            for c in 0..self.matrix[0].len() {
                let counting = self.count_surrounding(r, c);
                new_matrix[r][c] = counting.change_center(self.matrix[r][c]);
            }
        }
        self.matrix = new_matrix;
    }

    fn count_surrounding(&self, r: usize, c: usize) -> Counting {
        let mut count: Counting = Default::default();
        if r > 0 {
            if c > 0 {
                count.add_cell(self.matrix[r-1][c-1]);
            }
            count.add_cell(self.matrix[r-1][c]);
            if c < self.matrix[0].len() - 1 {
                count.add_cell(self.matrix[r-1][c+1]);
            }
        }
        if c > 0 {
            count.add_cell(self.matrix[r][c-1]);
        }
        if c < self.matrix[0].len() - 1 {
            count.add_cell(self.matrix[r][c+1]);
        }
        if r < self.matrix.len() - 1 {
            if c > 0 {
                count.add_cell(self.matrix[r+1][c-1]);
            }
            count.add_cell(self.matrix[r+1][c]);
            if c < self.matrix[0].len() - 1 {
                count.add_cell(self.matrix[r+1][c+1]);
            }
        }
        count
    }

    fn get_score(&self) -> usize {
        calc_score(&self.matrix)
    }

    fn print_score(&self) {

        println!("{}", self.get_score());
    }
}

fn calc_score(matrix: &Vec<Vec<char>>) -> usize {
    let mut tree = 0;
    let mut lumberyard = 0;
    for row in matrix.iter() {
        for &c in row {
            if c == '|' {
                tree += 1;
            } else if c == '#' {
                lumberyard += 1;
            }
        }
    }
    tree * lumberyard
}

#[derive(Debug, Copy, Clone, Default)]
struct Counting {
    open: usize,
    tree: usize,
    lumberyard: usize,
}

impl Counting {

    fn add_cell(&mut self, cell: char) {
        match cell {
            '.' => self.open += 1,
            '|' => self.tree += 1,
            '#' => self.lumberyard += 1,
            _ => panic!("unknown char {}", cell),
        }
    }

    fn change_center(&self, center: char) -> char {
        match center {
            '.' => if self.tree >= 3 { '|' } else { '.' },
            '|' => if self.lumberyard >= 3 { '#' } else { '|' },
            '#' => if self.lumberyard >= 1 && self.tree >= 1 { '#' } else { '.' },
            _ => panic!("unknown char {}", center),
        }
    }
}
