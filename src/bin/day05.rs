use std::io;
use std::io::prelude::*;

fn reacts_with(a:&u8, b:&u8) -> bool {
    if a < b {
        return b - a == 32;
    }
    a - b == 32
}

fn react(units: Vec<u8>) -> usize {
    let mut reacted: Vec<u8> = Vec::new();
    for unit in units.iter() {
        match reacted.last() {
            Some(prev_unit) => {
                if reacts_with(prev_unit, unit) {
                    reacted.pop();
                } else {
                    reacted.push(*unit);
                }
            },
            None => {
                reacted.push(*unit);
            }
        }
    }
    reacted.len()
}

fn strip_unit(units: Vec<u8>, unit: &u8) -> Vec<u8> {
    units.into_iter().filter(|u| *u != *unit && *u != *unit + 32).collect()
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
