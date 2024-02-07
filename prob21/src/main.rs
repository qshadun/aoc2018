use std::collections::HashSet;
use std::fs::read_to_string;

fn main() {
    let input = read_to_string("inputs/input21.txt").unwrap();
    let mut program = Program::from_input(&input);
    println!("{:?}", program);
    // it will run a couple of minutes for part2
    program.execute([0, 0, 0, 0, 0, 0]);
}

#[derive(Debug)]
struct Program {
    ip_pos: usize,
    instructions: Vec<Instruction>,
    r3_values: Vec<usize>,
    r3_values_set: HashSet<usize>,
}

impl Program {
    fn from_input(input: &str) -> Self {
        let lines: Vec<&str> = input.lines().collect();
        let parts: Vec<_> = lines[0].split(' ').collect();
        let ip_pos = parts[1].parse().unwrap();
        let mut instructions = vec![];
        for &line in &lines[1..] {
            instructions.push(Instruction::new(line));
        }
        Self {
            ip_pos,
            instructions,
            r3_values: vec![],
            r3_values_set: HashSet::new(),
        }
    }

    fn execute(&mut self, start_registers: [usize; 6]) -> [usize; 6] {
        let mut ip = start_registers[self.ip_pos];
        if ip > 0 {
            ip += 1;
        }
        let mut registers = start_registers.clone();
        let mut cnt = 0usize;
        while ip < self.instructions.len() {
            if ip == 28 {
                if self.r3_values_set.contains(&registers[3]) {
                    break;
                }
                self.r3_values.push(registers[3]);
                self.r3_values_set.insert(registers[3]);
                // uncomment this line for part1
                // break;
            }
            registers[self.ip_pos] = ip;

            eval(
                &self.instructions[ip].op,
                &mut registers,
                &self.instructions[ip].operands,
            );
            ip = registers[self.ip_pos] + 1;
            cnt += 1;
        }
        println!("==={} {:?} {}", ip, registers, cnt);
        println!("r3 values {:?}", self.r3_values);

        registers
    }
}
#[derive(Debug, Clone)]
struct Instruction {
    op: String,
    operands: [usize; 3],
}

impl Instruction {
    fn new(line: &str) -> Self {
        let parts: Vec<_> = line.split(" ").collect();
        let n1: String = parts[0].to_string();
        let n2: usize = parts[1].parse().unwrap();
        let n3: usize = parts[2].parse().unwrap();
        let n4: usize = parts[3].parse().unwrap();
        Self {
            op: n1,
            operands: [n2, n3, n4],
        }
    }
}

const ops: [&str; 16] = [
    "addr", "addi", "mulr", "muli", "banr", "bani", "borr", "bori", "setr", "seti", "gtir", "gtri",
    "gtrr", "eqir", "eqri", "eqrr",
];

fn eval(op: &str, registers: &mut [usize; 6], operands: &[usize; 3]) {
    let A = operands[0];
    let B = operands[1];
    let C = operands[2];
    match op {
        "addr" => {
            registers[C] = registers[A] + registers[B];
        }
        "addi" => {
            registers[C] = registers[A] + B;
        }
        "mulr" => {
            registers[C] = registers[A] * registers[B];
        }
        "muli" => {
            registers[C] = registers[A] * B;
        }
        "banr" => {
            registers[C] = registers[A] & registers[B];
        }
        "bani" => {
            registers[C] = registers[A] & B;
        }
        "borr" => {
            registers[C] = registers[A] | registers[B];
        }
        "bori" => {
            registers[C] = registers[A] | B;
        }
        "setr" => {
            registers[C] = registers[A];
        }
        "seti" => {
            registers[C] = A;
        }
        "gtir" => {
            registers[C] = if A > registers[B] { 1 } else { 0 };
        }
        "gtri" => {
            registers[C] = if registers[A] > B { 1 } else { 0 };
        }
        "gtrr" => {
            registers[C] = if registers[A] > registers[B] { 1 } else { 0 };
        }
        "eqir" => {
            registers[C] = if A == registers[B] { 1 } else { 0 };
        }
        "eqri" => {
            registers[C] = if registers[A] == B { 1 } else { 0 };
        }
        "eqrr" => {
            registers[C] = if registers[A] == registers[B] { 1 } else { 0 };
        }
        _ => {
            panic!("invalid opcode {op}");
        }
    }
}
