use std::io;
use std::io::prelude::*;

use std::vec::Vec;
use std::collections::HashMap;

struct Nap (u32, u32);

struct Guard {
    naps: Vec<Nap>,
}

impl Guard {
    fn sleepiest_minute(&self) -> (u32, u32) {
        let mut minutes: [u32; 60] = [0u32; 60];
        for nap in self.naps.iter() {
            for i in nap.0..nap.1 {
                minutes[i as usize] += 1;
            }
        }

        let mut sleepiest_minute = 0;
        let mut nap_count = 0;
        for i in 0..60 {
            if minutes[i] > nap_count {
                nap_count = minutes[i];
                sleepiest_minute = i;
            }
        }
        (sleepiest_minute as u32, nap_count)
    }

    fn total_nap_time(&self) -> u32 {
        self.naps.iter().fold(0, |acc, x| acc + x.1 - x.0)
    }

    fn new() -> Guard {
        Guard {
            naps: Vec::new()
        }
    }
}

fn main() {
    let mut input_lines: Vec<String> = Vec::new();
    let mut current_guard: u32 = 0;
    let mut falls_asleep: u32 = 0;
    let mut naps: HashMap<u32, Guard> = HashMap::with_capacity(50);

    for line in io::stdin().lock().lines() {
        input_lines.push(line.unwrap());
    }

    input_lines.sort_unstable();
    for line in input_lines {
        match line.chars().nth(19).unwrap() {
            'G' => {
                current_guard = line[26..].split(' ').next().unwrap().parse().unwrap();
                naps.entry(current_guard).or_insert(Guard::new());
            },
            'f' => {
                falls_asleep = line[15..17].parse().unwrap();
            },
            'w' => {
                let wakes_up = line[15..17].parse().unwrap();
                naps.get_mut(&current_guard).unwrap().naps.push(Nap (falls_asleep, wakes_up));
            },
            _ => {
                panic!("Character not expected");
            },
        }
    }

    let mut max_nap_time_total = 0;
    let mut sleepiest_guard_total = 0;
    let mut max_nap_time_minute = 0;
    let mut sleepiest_guard_minute = 0;
    let mut sleepiest_minute = 0;

    for (guard_id, guard) in naps.iter() {
        let total_nap_time = guard.total_nap_time();
        if total_nap_time > max_nap_time_total {
            sleepiest_guard_total = *guard_id;
            max_nap_time_total = total_nap_time;
        }

        let (_sleepiest_minute, minute_nap_time) = guard.sleepiest_minute();
        if minute_nap_time > max_nap_time_minute {
            sleepiest_guard_minute = *guard_id;
            max_nap_time_minute = minute_nap_time;
            sleepiest_minute = _sleepiest_minute;
        }
    }

    let (sleepiest_minute_total, _nap_count) = naps.get(&sleepiest_guard_total).unwrap().sleepiest_minute();

    println!("Sleepiest minute times ID of sleepiest guard: {}", sleepiest_guard_total * sleepiest_minute_total);

    println!("Sleepiest minute times ID of most regularly asleep guard: {}", sleepiest_guard_minute * sleepiest_minute);
}
