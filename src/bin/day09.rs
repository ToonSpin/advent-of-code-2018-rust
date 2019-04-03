use std::io;
use std::io::prelude::*;

extern crate regex;
use regex::Regex;

#[macro_use]
extern crate intrusive_collections;
use intrusive_collections::{LinkedList, LinkedListLink};

struct Marble {
    link: LinkedListLink,
    value: u32,
}

intrusive_adapter!(MarbleAdapter = Box<Marble>: Marble { link: LinkedListLink });

fn make_marble(m: u32) -> Box<Marble> {
    Box::new(Marble {
        link: LinkedListLink::new(),
        value: m,
    })
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

    let mut circle = LinkedList::new(MarbleAdapter::new());
    circle.push_back(make_marble(0));
    let mut cursor = circle.front_mut();

    for current_marble in 1..=max_marble * 100 {
        if current_marble == max_marble {
            println!(
                "The player with the best score scored: {}",
                scores.iter().max().unwrap()
            );
        }

        let mut score: u32 = 0;

        if current_marble % 23 == 0 {
            score += current_marble;

            for _i in 0..7 {
                cursor.move_prev();
                if let None = cursor.get() {
                    cursor.move_prev();
                }
            }

            let m = cursor.get().unwrap();
            score += m.value;

            cursor.remove();
        } else {
            cursor.move_next();
            if let None = cursor.get() {
                cursor.move_next();
            }

            cursor.insert_after(make_marble(current_marble));
            cursor.move_next();
        }

        scores[(current_marble as usize % num_players) as usize] += score;
    }

    println!(
        "If the max marble were 100 times as large: {}",
        scores.iter().max().unwrap()
    );

    Ok(())
}
