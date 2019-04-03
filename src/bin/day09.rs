use std::io;
use std::io::prelude::*;

extern crate regex;
use regex::Regex;

struct MarbleCircle {
    circle: Vec<u32>,
    current_index: usize,
    next: u32,
}

impl MarbleCircle {
    fn new() -> MarbleCircle {
        MarbleCircle {
            circle: vec![0],
            current_index: 0,
            next: 1,
        }
    }

    fn place(&mut self) -> u32 {
        let mut score = 0;
        if self.next % 23 == 0 {
            score += self.next;
            let mut remove_pos: usize = self.current_index;
            while remove_pos < 7 {
                remove_pos += self.circle.len();
            }
            remove_pos -= 7;
            let removed_marble = self.circle.remove(remove_pos);
            score += removed_marble;
            self.current_index = remove_pos;
        } else {
            let new_pos = self.current_index + 2;
            let mut new_pos: usize = new_pos % self.circle.len();
            if new_pos == 0 {
                new_pos = self.circle.len();
            }

            self.circle.insert(new_pos, self.next);
            self.current_index = new_pos;
        }
        self.next += 1;
        score
    }
}

fn get_score(num_players: usize, max_marble: u32) -> u32 {
    let mut circle = MarbleCircle::new();
    let mut scores: Vec<u32> = Vec::with_capacity(num_players);

    for _i in 0..num_players {
        scores.push(0u32);
    }

    for i in 1..=max_marble {
        scores[i as usize % num_players] += circle.place();
    }

    *scores.iter().max().unwrap()
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

    println!(
        "The player with the best score scored: {}",
        get_score(num_players, max_marble)
    );
    println!(
        "If the max marble were 100 times as large: {}",
        get_score(num_players, max_marble * 100)
    );

    Ok(())
}
