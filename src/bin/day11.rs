use std::io;
use std::io::prelude::*;

use std::collections::HashMap;

struct PowerGrid {
    grid: [i32; 90000],
    memo: HashMap<usize, i32>,
}

impl PowerGrid {
    fn new(serial: i32) -> PowerGrid {
        let mut grid: [i32; 90000] = [0; 90000];
        for y in 0..300 {
            let mut rack_id = 10;

            for x in 0..300 {
                rack_id += 1;

                let mut power_level = rack_id * (rack_id * (y + 1) + serial);
                power_level = power_level / 100 % 10 - 5;

                grid[(y * 300 + x) as usize] = power_level;
            }
        }

        PowerGrid {
            grid: grid,
            memo: HashMap::with_capacity(15252015),
        }
    }

    fn get(&self, x: usize, y: usize) -> i32 {
        self.grid[(y - 1) * 300 + x - 1]
    }

    fn square(&mut self, x: usize, y: usize, side: usize) -> i32 {
        if let Some(sum) = self.memo.get(&(side * 90000 + y * 300 + x)) {
            return *sum;
        }

        let mut total = 0;
        if side <= 3 {
            for q in y..y + side {
                for p in x..x + side {
                    total += self.get(p, q);
                }
            }
        } else {
            total = self.square(x, y, side - 1) + self.square(x + 1, y + 1, side - 1)
                - self.square(x + 1, y + 1, side - 2)
                + self.get(x + side - 1, y)
                + self.get(x, y + side - 1);
        }
        self.memo.insert(side * 90000 + y * 300 + x, total);
        total
    }
}

fn main() -> io::Result<()> {
    let mut input: i32 = 0;
    for line in io::stdin().lock().lines() {
        input = line.unwrap().parse().unwrap();
        break;
    }

    let mut grid = PowerGrid::new(input);

    let mut max_sum = -10000;
    let mut max_3_square = (0, 0);
    let mut max_square = (0, 0, 0);

    for s in 3..=300 as usize {
        for y in 1..=300 - s + 1 as usize {
            for x in 1..=300 - s + 1 as usize {
                let square_sum = grid.square(x, y, s);

                if square_sum > max_sum {
                    max_sum = square_sum;
                    max_square = (x, y, s);
                }
            }
        }
        if s == 3 {
            max_3_square = (max_square.0, max_square.1);
        }
    }

    println!(
        "The X,Y coordinate of the most powerful 3x3 square: ({},{})",
        max_3_square.0, max_3_square.1
    );
    println!(
        "The X,Y,size identifier of the square with the largest total power: ({},{},{})",
        max_square.0, max_square.1, max_square.2
    );

    Ok(())
}
