
use lazy_static::lazy_static;
use regex::Regex;
use std::{fs::read_to_string, collections::{HashMap, HashSet}, hash::Hash};

fn main() {
    let input = read_to_string("inputs/input16.txt").unwrap();
    let input_lines: Vec<_> = input.lines().collect();
    // part1(&input_lines);
    part2(&input_lines);
}

fn part2(input_lines: &Vec<&str>) {
    let mut test_program_start = 0;
    for i in 3..input_lines.len() {
        if input_lines[i].is_empty() && input_lines[i-1].is_empty() && input_lines[i-2].is_empty() {
            test_program_start = i + 1;
            break;
        }
    }
    let mut opcode_dict: HashMap<usize, HashSet<usize>> = HashMap::new();
    let mut i = 0;
    while i < test_program_start {
        if input_lines[i].starts_with("Before") {
            let reg_input = parse_register(input_lines[i]);
            let instruction = parse_instruction(input_lines[i + 1]);
            let reg_output = parse_register(input_lines[i +  2]);
            let mut oprands = [0; 3];
            oprands[..3].copy_from_slice(&instruction[1..4]);
            let opcode = instruction[0];
            let mut possible_opcode_idx = opcode_dict.entry(opcode).or_default();
            if possible_opcode_idx.len() == 1 {
                i += 3;
                continue;
            }
            if possible_opcode_idx.is_empty() {
                for j in 0..16 {
                    possible_opcode_idx.insert(j);
                }
            }
            let mut validated_idx = HashSet::new();
            for &j in possible_opcode_idx.iter() {
                let op = ops[j];
                match eval(op, reg_input, oprands) {
                    Ok(result) => {
                        if result == reg_output {
                            validated_idx.insert(j);
                        }
                    }
                    Err(_) => {}
                }
            }
            opcode_dict.insert(opcode, validated_idx);
            i += 3;
        } else {
            i += 1;
        }
    }
    println!("{:?}", opcode_dict);

    let mut opcode_to_ops_idx = [17usize; 16];
    let mut indegree = [0; 16];
    let mut ops_idx_to_opcode: HashMap<usize, HashSet<usize>> = HashMap::new();

    for (&opcode, ops_idxes) in opcode_dict.iter() {
        for &ops_idx in ops_idxes.iter() {
            indegree[ops_idx] += 1;
            ops_idx_to_opcode.entry(ops_idx).or_default().insert(opcode);
        }
    }
    while opcode_to_ops_idx.iter().any(|&x| x == 17) {
        for (ops_idx, d) in indegree.into_iter().enumerate() {
            if d == 1 {
                let opcode = *ops_idx_to_opcode.get(&ops_idx).unwrap().iter().next().unwrap();
                opcode_to_ops_idx[opcode] = ops_idx;

                for idx in 0..16 {
                    if idx != ops_idx {
                        let mut possible_code = ops_idx_to_opcode.get_mut(&idx).unwrap();
                        if possible_code.contains(&opcode) {
                            possible_code.remove(&opcode);
                            indegree[idx] -= 1;
                        }
                    }
                    
                }
            }
        }
    }
    
    let mut registers: [usize; 4] = [0, 0, 0, 0];
    for i in test_program_start..input_lines.len() {
        let instruction = parse_instruction(input_lines[i]);
        
        let op = ops[opcode_to_ops_idx[instruction[0]]];
        let mut oprands = [0; 3];
        oprands[..3].copy_from_slice(&instruction[1..4]);
        
        match eval(op, registers, oprands) {
            Ok(result) => {
                registers = result;
            }
            Err(_) => {panic!("failed to eval {:?}", instruction); }
        }
    }
    println!("{}", registers[0]);
}

fn part1(input_lines: &Vec<&str>) {
    let mut i = 0;
    let mut ans = 0;
    while i < input_lines.len() {
        if input_lines[i].starts_with("Before") {
            let reg_input = parse_register(input_lines[i]);
            let instruction = parse_instruction(input_lines[i + 1]);
            let reg_output = parse_register(input_lines[i +  2]);
            let mut oprands = [0; 3];
            let mut cnt = 0;
            oprands[..3].copy_from_slice(&instruction[1..4]);
            for op in ops {
                match eval(op, reg_input, oprands) {
                    Ok(result) => {
                        if result == reg_output { 
                            cnt += 1;
                            if cnt >= 3 {
                                ans += 1;
                                break;
                            }
                        }
                    }
                    Err(_) => {}
                }
            }
            i += 3;
        } else {
            i += 1;
        }
    }
    println!("{}", ans);
}
fn check_op(input: &str, instruction: &str, output: &str) -> bool {

    false
}

fn parse_register(line: &str) -> [usize; 4] {
    lazy_static! {
        static ref RE: Regex = Regex::new(
            r"[^\[]*\[(?P<n1>[-0-9]+),\s*(?P<n2>[-0-9]+),\s*(?P<n3>[-0-9]+),\s*(?P<n4>[-0-9]+)]"
        )
        .unwrap();
    }
    let caps = RE.captures(line).unwrap();
    let n1: usize = caps["n1"].parse().unwrap();
    let n2: usize = caps["n2"].parse().unwrap();
    let n3: usize = caps["n3"].parse().unwrap();
    let n4: usize = caps["n4"].parse().unwrap();
    [n1, n2, n3, n4]
}

fn parse_instruction(line: &str) -> [usize; 4] {
    let parts: Vec<_> = line.split(" ").collect();
    let n1: usize = parts[0].parse().unwrap();
    let n2: usize = parts[1].parse().unwrap();
    let n3: usize = parts[2].parse().unwrap();
    let n4: usize = parts[3].parse().unwrap();
    [n1, n2, n3, n4]
}

const ops: [&str; 16] = ["addr", "addi", "mulr", "muli", "banr", "bani", "borr", "bori", "setr", "seti", "gtir", "gtri", "gtrr", "eqir", "eqri", "eqrr"];

fn eval(op: &str, registers: [usize; 4], oprands: [usize; 3]) -> Result<[usize; 4], String> {
    let mut ans = registers.clone();
    let [A, B, C] = oprands;
    if C > 3 { return Err(format!("invalid register index C {C}")); }
    match op {
        "addr" =>  {
            if A > 3 { return Err(format!("invalid register index A {A}")); }
            if B > 3 { return Err(format!("invalid register index B {B}")); }
            ans[C] = registers[A] + registers[B];
        }
        "addi" =>  {
            if A > 3 { return Err(format!("invalid register index C {A}")); }
            ans[C] = registers[A] + B;
        }
        "mulr" =>  {
            if A > 3 { return Err(format!("invalid register index A {A}")); }
            if B > 3 { return Err(format!("invalid register index B {B}")); }
            ans[C] = registers[A] * registers[B];
        }
        "muli" =>  {
            if A > 3 { return Err(format!("invalid register index A {A}")); }
            ans[C] = registers[A] * B;
        }
        "banr" =>  {
            if A > 3 { return Err(format!("invalid register index A {A}")); }
            if B > 3 { return Err(format!("invalid register index B {B}")); }
            ans[C] = registers[A] & registers[B];
        }
        "bani" =>  {
            if A > 3 { return Err(format!("invalid register index A {A}")); }
            ans[C] = registers[A] & B;
        }
        "borr" =>  {
            if A > 3 { return Err(format!("invalid register index A {A}")); }
            if B > 3 { return Err(format!("invalid register index B {B}")); }
            ans[C] = registers[A] | registers[B];
        }
        "bori" =>  {
            if A > 3 { return Err(format!("invalid register index A {A}")); }
            ans[C] = registers[A] | B;
        }
        "setr" =>  {
            if A > 3 { return Err(format!("invalid register index A {A}")); }
            
            ans[C] = registers[A];
        }
        "seti" =>  {
            ans[C] = A;
        }
        "gtir" =>  {
            
            if B > 3 { return Err(format!("invalid register index B {B}")); }
            ans[C] = if A > registers[B] { 1 } else { 0 };
        }
        "gtri" =>  {
            if A > 3 { return Err(format!("invalid register index A {A}")); }
            
            ans[C] = if registers[A] > B { 1 } else { 0 };
        }
        "gtrr" =>  {
            if A > 3 { return Err(format!("invalid register index A {A}")); }
            if B > 3 { return Err(format!("invalid register index B {B}")); }
            ans[C] = if registers[A] > registers[B] { 1 } else { 0 };
        }
        "eqir" =>  {
            
            if B > 3 { return Err(format!("invalid register index B {B}")); }
            ans[C] = if A == registers[B] { 1 } else { 0 };
        }
        "eqri" =>  {
            if A > 3 { return Err(format!("invalid register index A {A}")); }
            
            ans[C] = if registers[A] == B { 1 } else { 0 };
        }
        "eqrr" =>  {
            if A > 3 { return Err(format!("invalid register index A {A}")); }
            if B > 3 { return Err(format!("invalid register index B {B}")); }
            ans[C] = if registers[A] == registers[B] { 1 } else { 0 };
        }
        _ => { return Err(format!("invalid opcode {op}")); }
    }

    Ok(ans)
}