use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
    hash::Hash,
};

fn main() {
    let input: String = read_to_string("../inputs/input15.txt").unwrap();
    let mut game = Game::from_input(&input);
    game.play();
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Coordinate {
    x: usize, // row
    y: usize, // col
}

impl Coordinate {
    fn new(x: usize, y: usize) -> Self {
        Self { x: x, y: y }
    }
}

#[derive(Debug)]
struct Npc {
    tp: char, //G: globin, E: elf
    id: usize,
    coord: Coordinate,
    hp: i32,
}

impl Npc {
    fn new(tp: char, id: usize, coord: Coordinate, hp: i32) -> Self {
        Self {
            tp: tp,
            id: id,
            coord: coord,
            hp: hp,
        }
    }
}

#[derive(Debug)]
struct Game {
    board: Vec<Vec<char>>,
    npcs: HashMap<Coordinate, Npc>,
    npc_position: HashMap<usize, Coordinate>, // id to coord
    turn: usize,
    elf_count: i32,
    goblin_count: i32,
}

impl Game {
    fn from_input(input: &str) -> Self {
        let mut board = vec![];
        let mut npcs = HashMap::new();
        let mut npc_position = HashMap::new();
        let mut id = 0;
        let mut elf_count = 0;
        let mut goblin_count = 0;
        for line in input.lines() {
            let mut row = vec![];
            for c in line.chars() {
                if c == 'G' || c == 'E' {
                    let coord = Coordinate::new(board.len(), row.len());
                    let npc = Npc::new(c, id, coord, 200);
                    npcs.insert(coord, npc);
                    npc_position.insert(id, coord);
                    id += 1;
                    if c == 'G' {
                        goblin_count += 1;
                    } else {
                        elf_count += 1;
                    }
                }
                row.push(c);
            }
            board.push(row);
        }
        Self {
            board: board,
            npcs: npcs,
            npc_position: npc_position,
            turn: 0,
            elf_count: elf_count,
            goblin_count: goblin_count,
        }
    }

    fn play(&mut self) {
        loop {
            let mut npc_ids: Vec<usize> = self.npc_position.keys().map(|id| *id).collect();
            npc_ids.sort_by_key(|id| self.npc_position.get(id).unwrap());
            for idx in 0..npc_ids.len() {
                let cur_id = npc_ids[idx];
                let cur_pos = self.npc_position.get(&cur_id);
                if cur_pos.is_none() {
                    continue;
                }
                let cur_npc = self.npcs.get(cur_pos.unwrap()).unwrap();
                if self.find_target_in_range(cur_npc).is_none() {
                    if let Some(nex_pos) = self.find_move(cur_npc) {
                        let cur_pos = cur_npc.coord;
                        self.board[cur_pos.x][cur_pos.y] = '.';
                        self.board[nex_pos.x][nex_pos.y] = cur_npc.tp;
                        self.npc_position.insert(cur_npc.id, nex_pos);
                        let new_npc = Npc::new(cur_npc.tp, cur_npc.id, nex_pos, cur_npc.hp);
                        self.npcs.remove(&cur_pos);
                        self.npcs.insert(nex_pos, new_npc);
                    }
                }
                let cur_pos = self.npc_position.get(&cur_id);
                if cur_pos.is_none() {
                    continue;
                }
                let cur_npc = self.npcs.get(cur_pos.unwrap()).unwrap();

                if let Some(enenmy_cord) = self.find_target_in_range(cur_npc) {
                    let targe_npc = self.npcs.get_mut(&enenmy_cord).unwrap();
                    targe_npc.hp -= 3;
                    if targe_npc.hp <= 0 {
                        // dead
                        let (x, y) = (targe_npc.coord.x, targe_npc.coord.y);
                        self.board[x][y] = '.';
                        self.npc_position.remove(&targe_npc.id);
                        if targe_npc.tp == 'G' {
                            self.goblin_count -= 1;
                            if self.goblin_count == 0 {
                                self.print_score(idx == npc_ids.len() - 1);
                                return;
                            }
                        } else {
                            self.elf_count -= 1;
                            if self.elf_count == 0 {
                                self.print_score(idx == npc_ids.len() - 1);
                                return;
                            }
                        }
                        self.npcs.remove(&enenmy_cord);
                    }
                }
            }
            self.turn += 1;
        }
    }

    fn find_move(&self, npc: &Npc) -> Option<Coordinate> {
        let mut adj_posotions = self.get_near_coords(npc.coord);
        adj_posotions = adj_posotions
            .into_iter()
            .filter(|&co| self.board[co.x][co.y] == '.')
            .collect();

        if adj_posotions.is_empty() {
            return None;
        }
        if let Some(target_pos) = self.find_target_pos(npc) {
            let mut visited = HashSet::new();
            visited.insert(target_pos);
            let mut positions = vec![target_pos];
            while !positions.is_empty() {
                let mut next_positions = vec![];
                let mut reached = vec![];
                for co in positions {
                    let moves = self.get_near_coords(co);
                    for mv in moves {
                        if !visited.contains(&mv) && self.board[mv.x][mv.y] == '.' {
                            visited.insert(mv);
                            next_positions.push(mv);
                            if adj_posotions.contains(&mv) {
                                reached.push(mv);
                            }
                        }
                    }
                }
                if !reached.is_empty() {
                    reached.sort();
                    return Some(reached[0]);
                }
                positions = next_positions;
            }
            None
        } else {
            None
        }
    }

    fn find_target_pos(&self, npc: &Npc) -> Option<Coordinate> {
        let enemy_tp = Game::get_enemy_tp(npc.tp);
        let mut visited = HashSet::new();
        visited.insert(npc.coord);
        let mut positions = vec![npc.coord];
        while !positions.is_empty() {
            let mut next_positions = vec![];
            for co in positions {
                let moves = self.get_near_coords(co);
                for mv in moves {
                    if self.board[mv.x][mv.y] == enemy_tp {
                        return Some(mv);
                    }
                    if !visited.contains(&mv) && self.board[mv.x][mv.y] == '.' {
                        visited.insert(mv);
                        next_positions.push(mv);
                    }
                }
            }
            positions = next_positions;
        }
        None
    }

    fn get_near_coords(&self, coord: Coordinate) -> Vec<Coordinate> {
        let mut ans = vec![];
        let (x, y) = (coord.x, coord.y);
        if x > 0 {
            ans.push(Coordinate::new(x - 1, y));
        }
        if y > 0 {
            ans.push(Coordinate::new(x, y - 1));
        }
        if y < self.board[x].len() - 1 {
            ans.push(Coordinate::new(x, y + 1));
        }
        if x < self.board.len() - 1 {
            ans.push(Coordinate::new(x + 1, y));
        }
        ans
    }

    fn find_target_in_range(&self, cur_npc: &Npc) -> Option<Coordinate> {
        let enemy_tp = Game::get_enemy_tp(cur_npc.tp);
        let mut enemies = vec![];
        for in_range_coord in self.get_near_coords(cur_npc.coord) {
            if self.board[in_range_coord.x][in_range_coord.y] == enemy_tp {
                enemies.push(self.npcs.get(&in_range_coord).unwrap());
            }
        }
        if !enemies.is_empty() {
            enemies.sort_by_key(|e| (e.hp, e.coord));
            Some(enemies[0].coord)
        } else {
            None
        }
    }

    fn get_enemy_tp(tp: char) -> char {
        if tp == 'G' {
            'E'
        } else {
            'G'
        }
    }

    fn print_score(&self, is_last: bool) {
        let mut sum_hp = 0;
        for npc in self.npcs.values() {
            if npc.hp > 0 {
                sum_hp += npc.hp;
            }
        }
        let turn = if is_last { self.turn + 1 } else { self.turn };
        println!("{} * {} = {}", sum_hp, turn, sum_hp as usize * turn)
    }
}
