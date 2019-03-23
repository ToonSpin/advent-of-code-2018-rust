use std::io;
use std::io::prelude::*;

use std::collections::HashSet;
use std::collections::LinkedList;

fn main() {
    let mut sum: i32 = 0;
    let mut freqs: LinkedList<i32> = LinkedList::new();
    let mut found_freqs: HashSet<i32> = HashSet::new();
    let mut done: bool = false;

    for line in io::stdin().lock().lines() {
        let freq = line.unwrap().parse::<i32>().unwrap();
        sum += freq;
        freqs.push_back(freq);
    }
    println!("Sum of frequency changes: {}", sum);

    sum = 0;
    found_freqs.insert(sum);
    while !done {
        for freq in freqs.iter() {
            sum += freq;
            if found_freqs.contains(&sum) {
                done = true;
                break;
            }
            found_freqs.insert(sum);
        }
    }
    println!("First frequency to appear twice: {}", sum);
}
