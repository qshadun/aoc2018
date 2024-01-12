use std::fs::read_to_string;
use std::collections::HashSet;

fn main() {
    let input = read_to_string("inputs/input1.txt").unwrap();
    part1(&input);
    part2(&input);
}

fn part1(input: &str) {
    let mut ans = 0;

    // Iterate over the lines and print each line
    for line in input.lines() {
        ans += line.parse::<i32>().unwrap();
    }
    println!("{ans}");
}

fn part2(input: &str) {
    let mut ans = 0;
    let mut seen: HashSet<i32> = HashSet::new();
    seen.insert(0);
    let mut found = false;
    // Iterate over the lines and print each line
    while !found {
        for line in input.lines() {
            ans += line.parse::<i32>().unwrap();
            if seen.contains(&ans) {
                found = true;
                println!("{ans}");
                break;
            } else {
                seen.insert(ans);
            }
        }
    }
}
