use std::fs::read_to_string;

fn main() {
    let input = read_to_string("inputs/input2.txt").unwrap();
    part1(&input);
    part2(&input);
}

fn part1(input: &str) {
    let mut count2 = 0;
    let mut count3 = 0;
    // Iterate over the lines and print each line
    for line in input.lines() {
        let mut counter = vec![0; 26];
        for b in line.as_bytes() {
            let idx = b - 'a' as u8;
            counter[idx as usize] += 1;
        }
        let mut has_two = false;
        let mut has_three = false;
        for cnt in counter {
            if cnt == 2 && !has_two {
                has_two = true;
                count2 += 1;
            } else if cnt == 3 && !has_three {
                has_three = true;
                count3 += 1;
            }
        }
    }
    println!("{}", count2 * count3)
}

fn part2(input: &str) {
    let lines: Vec<_> = input.lines().collect();
    for i in 0..lines.len() {
        let l1 = lines[i].as_bytes();
        
        for j in i+1..lines.len() {
            let l2 = lines[j].as_bytes();
            if l2.len() != l1.len() {
                continue;
            }
            let mut diff_count = 0;
            let mut diff_pos = 0;
            for k in 0..l1.len() {
                if l1[k] != l2[k] {
                    diff_count += 1;
                    if diff_count > 1 {
                        break;
                    } else {
                        diff_pos = k;
                    }
                }
            }
            if diff_count == 1 {
                println!("{} {}", lines[i], lines[j]);
                println!("{}{}", &(lines[i])[0..diff_pos], &(lines[i])[diff_pos+1..]);
                return ();
            }
        }
    }
}
