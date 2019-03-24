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

fn main () -> io::Result<()> {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input)?;
    let input = input.trim();

    let units = Vec::from(input.as_bytes());
    let mut units = VecDeque::from(units);

    let mut done = false;
    while !done {
        let mut found = false;
        let mut i = 1;

        while i < units.len() {
            if reacts_with(units.get(i).unwrap(), units.get(i - 1).unwrap()) {
                units.remove(i - 1);
                units.remove(i - 1);
                found = true;
            } else {
                i += 1;
            }
        }

        if found == false {
            done = true;
        }
    }

    println!("Length after reaction: {}", units.len());
    Ok(())
}
