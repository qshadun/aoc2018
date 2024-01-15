use std::fs::read_to_string;

fn main() {
    let input = read_to_string("inputs/input13.txt").unwrap();
    let mut game = Game::from_input(&input);
    for line in game.board.iter() {
        let row: String = line.iter().collect();
        println!("{}", row);
    }
    for cart in game.carts.iter(){
        println!("{:?}", cart);
    }
    let res = game.play2();
    println!("{:?}", res);
}


#[derive(Debug, Clone, Copy)]
enum Turn {
    Left,
    Straight,
    Right,
}

impl Turn {
    fn next_turn(self) -> Self {
        match self {
            Self::Left => Self::Straight,
            Self::Straight => Self::Right,
            Self::Right => Self::Left,
        }
    }
}


#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn from_char(c: char) -> Option<Self> {
        if c == '<' {
            Some(Self::Left)
        } else if c == '>' {
            Some(Self::Right)
        } else if c == '^' {
            Some(Self::Up)
        } else if c == 'v' {
            Some(Self::Down)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Cart{
    pos: (usize, usize),
    direction: Direction,
    turn: Turn,
    crashed: bool,
}

impl Cart {
    fn new(pos: (usize, usize), direction: Direction) -> Self {
        Self { pos, direction, turn: Turn::Left, crashed: false }
    }
}

#[derive(Debug)]
struct Game {
    board: Vec<Vec<char>>,
    carts: Vec<Cart>,
}

impl Game {
    fn from_input(input: &str) -> Self {
        let mut board: Vec<Vec<char>> = vec![];
        let mut carts = vec![];
        for line in input.lines() {
            let mut row: Vec<char> = vec![];
            for c in line.chars() {
                match Direction::from_char(c) {
                    Some(dir) => {
                        carts.push(Cart::new((board.len(), row.len()), dir));
                        match dir {
                            Direction::Left | Direction::Right => row.push('-'),
                            _ => row.push('|'),
                        }
                    }
                    None => row.push(c),
                }
            }
            board.push(row);
        }
        Self {
            board,
            carts,
        }
    }

    fn play(&mut self) -> (usize, usize) {
        loop {
            self.carts.sort_by_key(|c| c.pos);
            for i in 0..self.carts.len() {
                let mut cart = self.carts[i];
                let (row, col) = cart.pos;
                if self.board[row][col] == '+' {
                    cart.direction = Self::do_turn(cart.direction, cart.turn);
                    cart.turn = cart.turn.next_turn(); 
                } else if self.board[row][col] == '/' ||  self.board[row][col] == '\\' {
                    cart.direction = Self::turn_corner( self.board[row][col], cart.direction); 
                }
                cart.pos = Self::next_position(cart.pos, cart.direction);
                self.carts[i] = cart;
                if self.check_collision(i) {
                    return (cart.pos.1, cart.pos.0)
                }
            }
        }
    }

    fn check_collision(&self, i: usize) -> bool {
        for j in 0..self.carts.len() {
            if i != j && self.carts[i].pos == self.carts[j].pos {
                return true;
            }
        }
        false
    }

    fn play2(&mut self) -> (usize, usize) {
        loop {
            
            self.carts = self.carts.iter().filter(|c| !c.crashed).map(|c| *c).collect();
            if self.carts.len() == 1 {
                return (self.carts[0].pos.1, self.carts[0].pos.0);
            }
            self.carts.sort_by_key(|c| c.pos);
            
            for i in 0..self.carts.len() {
                let mut cart = self.carts[i];
                if cart.crashed {
                    continue;
                }
                let (row, col) = cart.pos;
                if self.board[row][col] == '+' {
                    cart.direction = Self::do_turn(cart.direction, cart.turn);
                    cart.turn = cart.turn.next_turn(); 
                } else if self.board[row][col] == '/' ||  self.board[row][col] == '\\' {
                    cart.direction = Self::turn_corner( self.board[row][col], cart.direction); 
                }
                cart.pos = Self::next_position(cart.pos, cart.direction);
                self.carts[i] = cart;
                self.do_collide(i);
            }
        }
    }

    fn do_collide(&mut self, i: usize) {
        for j in 0..self.carts.len() {
            if j != i && !self.carts[j].crashed && self.carts[j].pos == self.carts[i].pos {
                self.carts[i].crashed = true;
                self.carts[j].crashed = true;
                break;
            }
        }

    }

    fn next_position(cur_pos: (usize, usize), direction: Direction) -> (usize, usize) {
        let (row, col) = cur_pos;
        match direction {
            Direction::Up => (row - 1, col),
            Direction::Down => (row + 1, col),
            Direction::Left => (row, col - 1),
            Direction::Right => (row, col + 1),
        }
    }

    fn do_turn(direction: Direction, turn: Turn) -> Direction {
        match direction {
            Direction::Up => {
                match turn {
                    Turn::Left => Direction::Left,
                    Turn::Straight => Direction::Up,
                    Turn::Right => Direction::Right,
                }
            },
            Direction::Down => {
                match turn {
                    Turn::Left => Direction::Right,
                    Turn::Straight => Direction::Down,
                    Turn::Right => Direction::Left,
                }
            },
            Direction::Left => {
                match turn {
                    Turn::Left => Direction::Down,
                    Turn::Straight => Direction::Left,
                    Turn::Right => Direction::Up,
                }
            },
            Direction::Right => {
                match turn {
                    Turn::Left => Direction::Up,
                    Turn::Straight => Direction::Right,
                    Turn::Right => Direction::Down,
                }
            },
        }
    }

    fn turn_corner(corner: char, direction: Direction) -> Direction {
        if corner == '\\' {
            match direction {
                Direction::Up => Direction::Left,
                Direction::Down => Direction::Right,
                Direction::Left => Direction::Up,
                Direction::Right => Direction::Down,
            }
        } else {
            match direction {
                Direction::Up => Direction::Right,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Down,
                Direction::Right => Direction::Up,
            }
        }
    }
    
}