use std::{collections::HashMap, os::macos::raw::stat};

fn main() {
    //     let state = "#..#.#..##......###...###";
    //     let rules_str = "...## => #
    // ..#.. => #
    // .#... => #
    // .#.#. => #
    // .#.## => #
    // .##.. => #
    // .#### => #
    // #.#.# => #
    // #.### => #
    // ##.#. => #
    // ##.## => #
    // ###.. => #
    // ###.# => #
    // ####. => #";
    let state = "..#..###...#####.#.#...####.#..####..###.##.#.#.##.#....#....#.####...#....###.###..##.#....#######";
    let rules_str = "..### => .
.##.# => #
#..#. => .
#.#.# => #
###.. => #
.#..# => .
##..# => #
.###. => #
..#.. => .
..... => .
##### => .
.#... => #
...#. => #
#...# => #
####. => .
.#### => .
##.## => #
...## => .
..##. => .
#.##. => .
#.... => .
.#.#. => .
..#.# => #
#.#.. => #
##... => #
##.#. => .
#..## => .
.##.. => .
#.### => .
....# => .
.#.## => #
###.# => #";

    let rules = Rules::from_rules_str(rules_str);
    println!("{:?}", rules);

    let mut state: Vec<char> = state.chars().collect();
    let mut cur_left = 0;
    for i in 0..1000 {
        let mut next_state: Vec<char> = vec![];
        let cur_len = state.len();
        let k = format!("...{}{}", state[0], state[1]);
        if rules.next_gen(&k) == '#' {
            cur_left -= 1;
            next_state.push('#');
        }
        let k = format!("..{}{}{}", state[0], state[1], state[2]);
        next_state.push(rules.next_gen(&k));
        let k = format!(".{}{}{}{}", state[0], state[1], state[2], state[3]);
        next_state.push(rules.next_gen(&k));
        for j in 2..cur_len - 2 {
            let k: String = (&state[j - 2..j + 3]).iter().collect();
            next_state.push(rules.next_gen(&k));
        }
        let k = format!(
            "{}{}{}{}.",
            state[cur_len - 4],
            state[cur_len - 3],
            state[cur_len - 2],
            state[cur_len - 1]
        );
        next_state.push(rules.next_gen(&k));
        let k = format!(
            "{}{}{}..",
            state[cur_len - 3],
            state[cur_len - 2],
            state[cur_len - 1]
        );
        next_state.push(rules.next_gen(&k));
        let k = format!("{}{}...", state[cur_len - 2], state[cur_len - 1]);
        if rules.next_gen(&k) == '#' {
            next_state.push('#');
        }
        let mut first_sharp = 0;
        while next_state[first_sharp] == '.' {
            first_sharp += 1;
        }
        cur_left += first_sharp as i32;
        state = (&next_state[first_sharp..]).to_vec();
        // let ss:String = state.iter().collect();
        // println!("{i} {cur_left} {ss}");

        let ss: String = state.iter().collect();

        let mut ans = 0;
        for i in 0..state.len() {
            if state[i] == '#' {
                ans += i as i32 + cur_left;
            }
        }
        println!("{i} {cur_left} {ans}");
    }
    let ss: String = state.iter().collect();
    println!("{cur_left} {ss}");
    let mut ans = 0;
    for i in 0..state.len() {
        if state[i] == '#' {
            ans += i as i32 + cur_left;
        }
    }
    println!("{ans}");
}

#[derive(Debug)]
struct Rules {
    rules: HashMap<String, char>,
}

impl Rules {
    fn from_rules_str(rules_str: &str) -> Self {
        let mut rules: HashMap<String, char> = HashMap::new();

        for line in rules_str.lines() {
            let parts: Vec<_> = line.split(" ").collect();
            let key = parts[0].to_string();
            let v = parts[2].chars().next().unwrap();
            rules.insert(key, v);
        }
        Self { rules }
    }

    fn next_gen(&self, pattern: &str) -> char {
        *self.rules.get(pattern).unwrap_or(&'.')
    }
}
