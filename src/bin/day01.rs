use std::io;
use std::io::prelude::*;

use std::collections::HashSet;

fn main() {
    let mut freqs = Vec::new();
    let mut found_freqs: HashSet<i32> = HashSet::with_capacity(150000);

    for line in io::stdin().lock().lines() {
        let freq = line.unwrap().parse::<i32>().unwrap();
        freqs.push(freq);
    }
    println!(
        "Sum of frequency changes: {}",
        freqs.iter().fold(0, |sum, val| sum + val)
    );

    let mut sum = 0;
    found_freqs.insert(sum);
    'calibrating: loop {
        for freq in freqs.iter() {
            sum += freq;
            if !found_freqs.insert(sum) {
                break 'calibrating;
            }
        }
    }
    println!("First frequency to appear twice: {}", sum);
}
