fn main() {
    let mut grid: Vec<Vec<i32>> = vec![vec![0; 301]; 301];
    for x in 1..=300 {
        for y in 1..=300 {
            grid[y][x] = pow_level(x as i32, y as i32, 3031);
        }
    }
    // part1(&grid);
    part2(&grid);
}

fn part2(grid: &Vec<Vec<i32>>) {
    let mut prefix_sum = vec![vec![0; 301]; 301];
    for x in 1..=300 {
        for y in 1..=300 {
            prefix_sum[y][x] = grid[y][x] + prefix_sum[y][x - 1] + prefix_sum[y-1][x] - prefix_sum[y-1][x-1];
        }
    }
    let mut max_x = 1;
    let mut max_y = 1;
    let mut max_size = 1;
    let mut max_pow = 0;
    for x in 1..=300 {
        for y in 1..=300 {
            let cur_max_size = 300 - x.max(y) + 1;
            for size in 0..cur_max_size {
                let pow = prefix_sum[y+size][x+size] - prefix_sum[y+size][x-1] - prefix_sum[y-1][x+size] + prefix_sum[y-1][x-1];
                if pow > max_pow {
                    max_pow = pow;
                    max_x = x;
                    max_y = y;
                    max_size = size+1;
                }
            }

        }
    }
    println!("{},{},{},{}", max_x, max_y, max_size, max_pow);
}

fn part1(grid: &Vec<Vec<i32>>) {
    let mut max_x = 1;
    let mut max_y = 1;
    let mut max_pow = 0;
    for x in 1..=298 {
        for y in 1..=298 {
            let mut pow = 0;
            for i in 0..3 {
                for j in 0..3 {
                    pow += grid[y+j][x+i];
                }
            }
            if pow > max_pow {
                max_pow = pow;
                max_x = x;
                max_y = y;
                
            }
        }
        
    }
    println!("{},{},{}", max_x, max_y, max_pow);
}
fn pow_level(x: i32, y: i32, serial_number: i32) -> i32 {
    let rack = x + 10;
    let mut ans = (rack * y + serial_number) * rack;
    (ans / 100) % 10 - 5
}

#[cfg(test)]
mod tests {
    use crate::pow_level;

    #[test]
    fn test_pow_level() {
        assert_eq!(pow_level(3, 5, 8), 4);
        assert_eq!(pow_level(122, 79, 57), -5);
        assert_eq!(pow_level(217, 196, 39), 0);
        assert_eq!(pow_level(101, 153, 71), 4);
    }
}