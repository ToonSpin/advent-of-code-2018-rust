use std::io;
use std::io::prelude::*;

struct PowerGrid {
    grid: [i32; 90000],
}

impl PowerGrid {
    fn new(serial: i32) -> PowerGrid {
        let mut powerlevels: [i32; 90000] = [0; 90000];
        for y in 0..300 as usize {
            let mut rack_id = 10;

            for x in 0..300 as usize {
                rack_id += 1;

                let mut power_level = rack_id * (rack_id * (y + 1) as i32 + serial);
                power_level = power_level / 100 % 10 - 5;

                powerlevels[y * 300 + x] = power_level;
            }
        }

        let mut summed_area: [i32; 90000] = [0; 90000];
        for y in 0..300 as usize {
            for x in 0..300 as usize {
                let mut sum = powerlevels[y * 300 + x];
                if x > 0 {
                    sum += summed_area[y * 300 + x - 1]
                }
                if y > 0 {
                    sum += summed_area[(y - 1) * 300 + x]
                }
                if x > 0 && y > 0 {
                    sum -= summed_area[(y - 1) * 300 + x - 1]
                }

                summed_area[y * 300 + x] = sum;
            }
        }

        PowerGrid { grid: summed_area }
    }

    fn get(&self, x: usize, y: usize) -> i32 {
        self.grid[y * 300 + x]
    }

    fn square(&self, x: usize, y: usize, side: usize) -> i32 {
        let mut sum = self.get(x + side - 1, y + side - 1);
        if x > 0 {
            sum -= self.get(x - 1, y + side - 1)
        }
        if y > 0 {
            sum -= self.get(x + side - 1, y - 1)
        }
        if x > 0 && y > 0 {
            sum += self.get(x - 1, y - 1)
        }
        sum
    }
}

fn main() -> io::Result<()> {
    let mut input: i32 = 0;
    for line in io::stdin().lock().lines() {
        input = line.unwrap().parse().unwrap();
        break;
    }

    let grid = PowerGrid::new(input);

    let mut max_sum = -10000;
    let mut max_3_square = (0, 0);
    let mut max_square = (0, 0, 0);

    for s in 3..=300 as usize {
        for y in 0..300 - s as usize {
            for x in 0..300 - s as usize {
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
        max_3_square.0 + 1,
        max_3_square.1 + 1
    );
    println!(
        "The X,Y,size identifier of the square with the largest total power: ({},{},{})",
        max_square.0 + 1,
        max_square.1 + 1,
        max_square.2
    );

    Ok(())
}
