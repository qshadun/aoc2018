use core::num;
use std::{cell::RefCell, rc::Rc};

fn main() {
    //491 players; last marble is worth 71058 points

    // part1(491, 71058);
    part2(491, 7105800);
}

// type Link = Option<Rc<RefCell<Node>>>;
// struct Node {
//     val: usize,
//     prev: Link,
//     next: Link,
// }

// impl Node {
//     fn new(val: usize, prev: Link, next: Link) -> Self {
//         Self { val: val, prev: prev, next: next }
//     }
// }
type MarbleID = usize;
type MarbleValue = usize;

#[derive(Debug)]
struct Marble {
    val: MarbleValue,
    prev: MarbleID,
    next: MarbleID,
}

impl Marble {
    fn new(val: MarbleValue) -> Self {
        Self {
            val: val,
            prev: 0,
            next: 0,
        }
    }
}

struct Circle {
    marbles: Vec<Marble>,
    current: MarbleID,
}

impl Circle {
    fn new() -> Self {
        Self {
            marbles: vec![Marble::new(0)],
            current: 0,
        }
    }

    fn add_marble(&mut self, value: MarbleValue) -> MarbleID {
        let id = self.marbles.len();
        self.marbles.push(Marble::new(value));
        id
    }

    fn insert_after(&mut self, to_insert: MarbleID, after: MarbleID) {
        let old_next = self.marbles[after].next;
        self.marbles[after].next = to_insert;
        self.marbles[old_next].prev = to_insert;
        self.marbles[to_insert].prev = after;
        self.marbles[to_insert].next = old_next;
    }

    fn remove(&mut self, id: MarbleID) {
        let (prev, next) = (self.marbles[id].prev, self.marbles[id].next);
        self.marbles[prev].next = next;
        self.marbles[next].prev = prev;
    }

    fn clockwise(&self, mut i: usize) -> MarbleID {
        let mut id = self.current;
        while i > 0 {
            id = self.marbles[id].next;
            i -= 1;
        }
        id
    }

    fn counter_clockwise(&self, mut i: usize) -> MarbleID {
        let mut id = self.current;
        while i > 0 {
            id = self.marbles[id].prev;
            i -= 1;
        }
        id
    }
}

use std::fmt;
impl fmt::Debug for Circle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut id = self.current;
        loop {
            let m = &self.marbles[id];
            write!(f, "{} ", m.val)?;
            id = m.next;
            if id == self.current {
                break;
            }
        }
        Ok(())
    }
}
fn part2(num_of_players: usize, last_marble: usize) -> usize {
    let mut circle = Circle::new();
    let mut scores = vec![0; num_of_players];
    for marble_value in 1..=last_marble {
        if marble_value % 23 != 0 {
            let new_id = circle.add_marble(marble_value);
            let mut before = circle.clockwise(1);
            circle.insert_after(new_id, before);
            circle.current = new_id;
        } else {
            let cur_player = marble_value % num_of_players;
            let remove_id = circle.counter_clockwise(7);
            scores[cur_player] += marble_value + circle.marbles[remove_id].val;
            circle.remove(remove_id);
            circle.current = circle.marbles[remove_id].next;
        }
    }

    let ans = scores.iter().max().unwrap();
    println!("{}", ans);
    *ans
}

fn part1(num_of_players: usize, last_marble: usize) -> usize {
    let mut board = vec![0];
    let mut cur_pos = 1usize;
    let mut scores = vec![0; num_of_players];

    for marble in 1..=last_marble {
        if marble % 23 != 0 {
            let next_pos = (cur_pos + 1) % board.len();
            cur_pos = next_pos + 1;
            board.insert(cur_pos, marble);
        } else {
            let cur_player = marble % num_of_players;
            let remove_pos = (cur_pos + board.len() - 7) % board.len();
            scores[cur_player] += marble + board[remove_pos];
            board.remove(remove_pos);
            cur_pos = remove_pos;
        }
    }
    let ans = scores.iter().max().unwrap();
    println!("{}", ans);
    *ans
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        /*
        9 players, last marble 25, high score 32
        10 players; last marble is worth 1618 points: high score is 8317
        13 players; last marble is worth 7999 points: high score is 146373
        17 players; last marble is worth 1104 points: high score is 2764
        21 players; last marble is worth 6111 points: high score is 54718
        30 players; last marble is worth 5807 points: high score is 37305

         */
        assert_eq!(part1(9, 25), 32);
        assert_eq!(part1(10, 1618), 8317);
        assert_eq!(part1(13, 7999), 146373);
        assert_eq!(part1(17, 1104), 2764);
        assert_eq!(part1(21, 6111), 54718);
        assert_eq!(part1(30, 5807), 37305);
    }
}
