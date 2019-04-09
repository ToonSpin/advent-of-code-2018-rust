use std::io;
use std::io::prelude::*;

extern crate regex;
use regex::Regex;

struct Marble {
    value: u32,
    next: usize,
    prev: usize,
}

struct MarbleCircle {
    circle: Vec<Marble>,
    current_marble: usize,
}

impl MarbleCircle {
    fn new() -> MarbleCircle {
        let mut v = Vec::with_capacity(8388608);
        v.push(Marble {
            value: 0,
            next: 0,
            prev: 0,
        });
        MarbleCircle {
            circle: v,
            current_marble: 0,
        }
    }

    fn clockwise(&mut self, n: u8) {
        for _i in 0..n {
            self.current_marble = self.circle[self.current_marble].next;
        }
    }

    fn counterclockwise(&mut self, n: u8) {
        for _i in 0..n {
            self.current_marble = self.circle[self.current_marble].prev;
        }
    }

    fn insert_after_current(&mut self, value: u32) {
        let next_marble: usize = self.circle[self.current_marble].next;
        let new_marble: usize = self.circle.len();

        self.circle[self.current_marble].next = new_marble;
        self.circle[next_marble].prev = new_marble;

        self.circle.push({
            Marble {
                value,
                prev: self.current_marble,
                next: next_marble,
            }
        })
    }

    fn remove_before_current(&mut self) -> u32 {
        let marble_to_remove: usize = self.circle[self.current_marble].prev;
        let new_prev_marble: usize = self.circle[marble_to_remove].prev;

        self.circle[self.current_marble].prev = new_prev_marble;
        self.circle[new_prev_marble].next = self.current_marble;

        self.circle[marble_to_remove].value
    }
}

fn main() -> io::Result<()> {
    let mut num_players: usize = 0;
    let mut max_marble: u32 = 0;

    let re = Regex::new(r"^(\d+)\D+(\d+)\D+$").unwrap();
    for line in io::stdin().lock().lines() {
        if let Some(caps) = re.captures(line.unwrap().as_str()) {
            num_players = caps[1].parse().unwrap();
            max_marble = caps[2].parse().unwrap();
        }
    }
    let mut scores: Vec<u32> = Vec::with_capacity(num_players);
    for _i in 0..num_players {
        scores.push(0u32);
    }

    let mut circle = MarbleCircle::new();

    for next_marble in 1..=max_marble * 100 {
        if next_marble == max_marble {
            println!(
                "The player with the best score scored: {}",
                scores.iter().max().unwrap()
            );
        }

        let mut score: u32 = 0;

        if next_marble % 23 == 0 {
            score += next_marble;
            circle.counterclockwise(6);
            score += circle.remove_before_current();
        } else {
            circle.clockwise(1);
            circle.insert_after_current(next_marble);
            circle.clockwise(1);
        }

        scores[(next_marble as usize % num_players) as usize] += score;
    }

    println!(
        "If the max marble were 100 times as large: {}",
        scores.iter().max().unwrap()
    );

    Ok(())
}
