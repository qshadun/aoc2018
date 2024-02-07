use std::fs::read_to_string;

fn main() {
    let input = read_to_string("inputs/input5.txt").unwrap();
    part1(&input);
    part2(&input);
}

fn part1(input: &str) {
    println!("{}", react(input));
}

fn react(input: &str) -> usize {
    let mut ans: Vec<char> = vec![];

    for c in input.chars() {
        match ans.last() {
            None => ans.push(c),
            Some(&c1) => {
                if c.is_uppercase() && c1.is_lowercase() && c.to_ascii_lowercase() == c1 {
                    ans.pop();
                } else if c.is_lowercase() && c1.is_uppercase() && c1.to_ascii_lowercase() == c {
                    ans.pop();
                } else {
                    ans.push(c);
                }
            }
        }
    }
    ans.len()
}

fn part2(input: &str) {
    let mut ans = usize::MAX;
    for i in 0..26 {
        let lower: char = (b'a' + i) as char;
        let upper = lower.to_ascii_uppercase();
        let reduced: String = input
            .chars()
            .filter(|&c| c != lower && c != upper)
            .collect();
        ans = ans.min(react(&reduced));
    }
    println!("{}", ans);
}
