fn main() {
    let mut game = Game::new(505961);
    game.play();
}

struct Game {
    input: Vec<usize>,
    recipes: Vec<usize>,
}

impl Game {
    fn new(input: usize) -> Self {
        let mut iv = vec![];
        let mut ii = input;
        while ii > 0 {
            iv.push(ii % 10);
            ii /= 10;
        }
        iv.reverse();
        Self {
            input: iv,
            recipes: vec![3, 7],
        }
    }

    fn play(&mut self) {
        loop {
            let mut pos1 = 0usize;
            let mut pos2 = 1usize;
            loop {
                let sum = self.recipes[pos1] + self.recipes[pos2];
                if sum > 9 {
                    self.recipes.push(1);
                    if self.check() {
                        return;
                    }
                    self.recipes.push(sum - 10);
                    if self.check() {
                        return;
                    }
                } else {
                    self.recipes.push(sum);
                    if self.check() {
                        return;
                    }
                }
                pos1 = (pos1 + self.recipes[pos1] + 1) % self.recipes.len();
                pos2 = (pos2 + self.recipes[pos2] + 1) % self.recipes.len();
            }
        }
    }

    fn check(&self) -> bool {
        if self.recipes.len() < self.input.len() {
            return false;
        }
        for i in 1..=self.input.len() {
            if self.input[self.input.len() - i] != self.recipes[self.recipes.len() - i] {
                return false;
            }
        }
        println!("{}", self.recipes.len() - self.input.len());
        true
    }
}
fn part1() {
    let input = 505961;

    let mut recipes: Vec<usize> = vec![3, 7];
    let mut pos1 = 0usize;
    let mut pos2 = 1usize;
    while recipes.len() < input + 10 {
        let sum = recipes[pos1] + recipes[pos2];
        if sum > 9 {
            recipes.push(1);
            recipes.push(sum - 10);
        } else {
            recipes.push(sum);
        }
        pos1 = (pos1 + recipes[pos1] + 1) % recipes.len();
        pos2 = (pos2 + recipes[pos2] + 1) % recipes.len();
    }
    let ans: String = recipes[input..input + 10]
        .iter()
        .map(|x| x.to_string())
        .collect();
    println!("{ans}");
}
