use std::fs::read_to_string;

fn main() {
    let input = read_to_string("inputs/input8.txt").unwrap();
    // let input = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2";
    let input: Vec<usize> = input.split(" ").map(|s| s.parse().unwrap()).collect();
    part1(&input);
    part2(&input);
}

fn part1(input: &Vec<usize>) {
    let mut stack: Vec<(usize, usize)> = vec![];
    let mut ans = 0;
    stack.push((input[0], input[1]));
    let mut i = 2;
    while let Some((children_size, meta_size)) = stack.pop() {
        if children_size > 0 {
            stack.push((children_size - 1, meta_size));
            stack.push((input[i], input[i + 1]));
            i += 2;
        } else {
            for j in 0..meta_size {
                ans += input[i + j];
            }
            i += meta_size;
        }
    }
    println!("{}", ans);
}

fn part2(input: &Vec<usize>) {
    let mut stack: Vec<Node> = vec![];
    stack.push(Node::new(input[0], input[1]));
    let mut i = 2;
    while i < input.len() {
        let cur = stack.last_mut().unwrap();
        if cur.unhandled_children_size == 0 {
            let mut val = 0;
            if cur.children_size == 0 {
                for j in 0..cur.meta_size {
                    val += input[i + j];
                }
            } else {
                for j in 0..cur.meta_size {
                    let ci = input[i + j];
                    if ci == 0 || ci > cur.children_val.len() {
                        continue;
                    } else {
                        val += cur.children_val[ci - 1];
                    }
                }
            }
            i += cur.meta_size;
            if i == input.len() {
                println!("{}", val);
                return;
            }
            stack.pop();
            let parent = stack.last_mut().unwrap();
            parent.unhandled_children_size -= 1;
            parent.children_val.push(val);
        } else {
            stack.push(Node::new(input[i], input[i + 1]));
            i += 2;
        }
    }
}

struct Node {
    children_size: usize,
    unhandled_children_size: usize,
    meta_size: usize,
    children_val: Vec<usize>,
}

impl Node {
    fn new(children_size: usize, meta_size: usize) -> Self {
        Self {
            children_size,
            unhandled_children_size: children_size,
            meta_size,
            children_val: vec![],
        }
    }
}
