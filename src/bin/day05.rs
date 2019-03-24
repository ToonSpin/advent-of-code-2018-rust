use std::io;
use std::io::prelude::*;

use std::collections::VecDeque;

fn reacts_with(a:&u8, b:&u8) -> bool {
    if a < b && b - a == 32 {
        return true;
    }
    if b < a && a - b == 32 {
        return true;
    }
    false
}

fn react(mut units: Vec<u8>) -> usize {
    loop {
        let mut done = true;
        let mut i = 1;

        while i < units.len() {
            if reacts_with(units.get(i).unwrap(), units.get(i - 1).unwrap()) {
                units.remove(i - 1);
                units.remove(i - 1);
                done = false;
            } else {
                i += 1;
            }
        }

        if done {
            return units.len();
        }
    }
}

fn strip_unit(mut units: Vec<u8>, unit: &u8) -> Vec<u8> {
    let mut i = 0;
    loop {
        match units.get(i) {
            Some(u) => {
                if *u == *unit || *u == unit + 32 {
                    units.remove(i);
                } else {
                    i += 1;
                }
            },
            None => { return units; },
        }
    }
}

fn main () -> io::Result<()> {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input)?;
    let input = input.trim();

    let units = Vec::from(input.as_bytes());

    println!("Length after reaction: {}", react(Vec::from(units.clone())));

    for unit in "ABCDEFGHIJKLMNOPQRSTUVWXYZ".as_bytes() {
        let stripped = strip_unit(Vec::from(units.clone()), unit);
        println!("Length after reaction after stripping {}: {}", *unit as char, react(stripped));
    }

    Ok(())
}
