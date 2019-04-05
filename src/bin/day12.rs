use std::io;
use std::io::prelude::*;

use std::collections::VecDeque;

struct PotRow {
    state: VecDeque<bool>,
    number_of_first_pot: i64,
    rules: [bool; 32],
}

impl PotRow {
    fn new(initial_state: &[u8], rules: [bool; 32]) -> PotRow {
        let mut state: VecDeque<bool> = VecDeque::new();
        for p in initial_state.iter() {
            let p = if *p == b'#' { true } else { false };
            state.push_back(p);
        }
        PotRow {
            state,
            number_of_first_pot: 0,
            rules,
        }
    }

    fn expand(&mut self) {
        for _i in 0..=3 {
            self.number_of_first_pot -= 1;
            self.state.push_front(false);
            self.state.push_back(false);
        }
    }

    fn trim(&mut self) {
        while *(self.state.front().unwrap()) == false {
            self.number_of_first_pot += 1;
            self.state.pop_front();
        }

        while *(self.state.back().unwrap()) == false {
            self.state.pop_back();
        }
    }

    fn sum_of_numbers(&self) -> i64 {
        let mut sum = 0;
        let mut pot_number = self.number_of_first_pot;
        for p in self.state.iter() {
            if *p {
                sum += pot_number;
            }
            pot_number += 1;
        }
        sum
    }

    fn iterate(&mut self) {
        self.expand();

        let mut new_state = VecDeque::with_capacity(self.state.capacity());
        let l = self.state.len();

        let mut key: usize = 0;
        for i in 0..5 {
            key *= 2;
            if self.state[i] {
                key += 1;
            }
        }

        for i in 0..l - 5 {
            new_state.push_back(self.rules[key]);
            key %= 16;
            key *= 2;
            if self.state[i + 5] {
                key += 1;
            }
        }
        self.state = new_state;
        self.number_of_first_pot += 2;
        self.trim();
    }
}

fn main() -> io::Result<()> {
    let lines = io::stdin();
    let mut lines = lines.lock().lines();

    let initial_state = lines.next().unwrap().unwrap();

    lines.next();
    let mut rules = [false; 32];

    for line in lines {
        let line = line.unwrap();
        let line = line.as_bytes();

        let mut key: usize = 0;
        if line[9] == b'#' {
            for i in 0..5 {
                key *= 2;
                if line[i] == b'#' {
                    key += 1;
                }
            }
            rules[key] = true;
        }
    }

    let mut row = PotRow::new(&initial_state.as_bytes()[15..], rules);
    let mut last_state = row.state.clone();
    let mut last_sum = row.sum_of_numbers();

    let mut i = 1;
    loop {
        row.iterate();

        let sum = row.sum_of_numbers();

        if i == 20 {
            println!("The sum of all pot numbers after 20 iterations: {}", sum);
        }

        if last_state == row.state {
            println!(
                "The sum of all pot numbers after 50 billion iterations: {}",
                sum + (50000000000 - i) * (sum - last_sum)
            );
            break;
        }

        last_state = row.state.clone();
        last_sum = sum;
        i += 1;
    }

    Ok(())
}
