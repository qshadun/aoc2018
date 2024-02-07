use std::fs::read_to_string;

fn main() {
    let input = read_to_string("inputs/input19.txt").unwrap();
    let program = Program::from_input(&input);
    println!("{:?}", program);
    // for part 1, just uncomment below line
    //program.execute([0, 0, 0, 0, 0, 0]);
    // for part2, uncomment below line, run to the part when reg0 changed to 0, find n2 in reg1
    // the ans is the sum of all factors of reg1
    // program.execute([1, 0, 0, 0, 0, 0]);
    let n2 = 10551309;
    let n1 = 909;
    let ans1: usize = factors(n1).iter().sum();
    let ans2: usize = factors(n2).iter().sum();
    println!("p1 {} p2 {}", ans1, ans2);
}

fn factors(n: usize) -> Vec<usize> {
    let mut ans = vec![];
    for i in 1..=n {
        if n % i == 0 {
            ans.push(i);
        }
    }
    ans
}
#[derive(Debug)]
struct Program {
    ip_pos: usize,
    instructions: Vec<Instruction>,
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
        }
    }

    fn execute(&self, start_registers: [usize; 6]) -> [usize; 6] {
        let mut ip = 0;
        let mut registers = start_registers.clone();
        let mut cnt = 0usize;
        while ip < self.instructions.len() {
            let old_r0 = registers[0];
            registers[self.ip_pos] = ip;
            let instruction = self.instructions[ip].clone();
            registers = eval(&instruction.op, registers, instruction.operands).unwrap();
            if old_r0 != registers[0] {
                println!("{} {:?} old_r0 {}, cnt {}", ip, registers, old_r0, cnt);
            }
            ip = registers[self.ip_pos] + 1;
            cnt += 1;
        }
        println!("==={} {:?} {}", ip, registers, cnt);
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

fn eval(op: &str, registers: [usize; 6], operands: [usize; 3]) -> Result<[usize; 6], String> {
    let mut ans = registers.clone();
    let [A, B, C] = operands;
    if C > 5 {
        return Err(format!("invalid register index C {C}"));
    }
    match op {
        "addr" => {
            if A > 5 {
                return Err(format!("invalid register index A {A}"));
            }
            if B > 5 {
                return Err(format!("invalid register index B {B}"));
            }
            ans[C] = registers[A] + registers[B];
        }
        "addi" => {
            if A > 5 {
                return Err(format!("invalid register index C {A}"));
            }
            ans[C] = registers[A] + B;
        }
        "mulr" => {
            if A > 5 {
                return Err(format!("invalid register index A {A}"));
            }
            if B > 5 {
                return Err(format!("invalid register index B {B}"));
            }
            ans[C] = registers[A] * registers[B];
        }
        "muli" => {
            if A > 5 {
                return Err(format!("invalid register index A {A}"));
            }
            ans[C] = registers[A] * B;
        }
        "banr" => {
            if A > 5 {
                return Err(format!("invalid register index A {A}"));
            }
            if B > 5 {
                return Err(format!("invalid register index B {B}"));
            }
            ans[C] = registers[A] & registers[B];
        }
        "bani" => {
            if A > 5 {
                return Err(format!("invalid register index A {A}"));
            }
            ans[C] = registers[A] & B;
        }
        "borr" => {
            if A > 5 {
                return Err(format!("invalid register index A {A}"));
            }
            if B > 5 {
                return Err(format!("invalid register index B {B}"));
            }
            ans[C] = registers[A] | registers[B];
        }
        "bori" => {
            if A > 5 {
                return Err(format!("invalid register index A {A}"));
            }
            ans[C] = registers[A] | B;
        }
        "setr" => {
            if A > 5 {
                return Err(format!("invalid register index A {A}"));
            }

            ans[C] = registers[A];
        }
        "seti" => {
            ans[C] = A;
        }
        "gtir" => {
            if B > 5 {
                return Err(format!("invalid register index B {B}"));
            }
            ans[C] = if A > registers[B] { 1 } else { 0 };
        }
        "gtri" => {
            if A > 5 {
                return Err(format!("invalid register index A {A}"));
            }

            ans[C] = if registers[A] > B { 1 } else { 0 };
        }
        "gtrr" => {
            if A > 5 {
                return Err(format!("invalid register index A {A}"));
            }
            if B > 5 {
                return Err(format!("invalid register index B {B}"));
            }
            ans[C] = if registers[A] > registers[B] { 1 } else { 0 };
        }
        "eqir" => {
            if B > 5 {
                return Err(format!("invalid register index B {B}"));
            }
            ans[C] = if A == registers[B] { 1 } else { 0 };
        }
        "eqri" => {
            if A > 5 {
                return Err(format!("invalid register index A {A}"));
            }

            ans[C] = if registers[A] == B { 1 } else { 0 };
        }
        "eqrr" => {
            if A > 5 {
                return Err(format!("invalid register index A {A}"));
            }
            if B > 5 {
                return Err(format!("invalid register index B {B}"));
            }
            ans[C] = if registers[A] == registers[B] { 1 } else { 0 };
        }
        _ => {
            return Err(format!("invalid opcode {op}"));
        }
    }

    Ok(ans)
}
