use std::io;
use std::io::prelude::*;

use std::collections::HashMap;
use std::collections::VecDeque;

fn box_id_diff(a: &String, b: &String) -> i32 {
    let a = a.as_bytes();
    let b = b.as_bytes();

    let length: usize = a.len();
    let mut diff: i32 = 0;

    for i in 0..length {
        if a[i] != b[i] {
            diff += 1;
        }
    }

    diff
}

fn common_bytes(a: &String, b: &String) -> String {
    let a = a.as_bytes();
    let b = b.as_bytes();

    let length: usize = a.len();
    let mut result = String::new();

    for i in 0..length {
        if a[i] == b[i] {
            result.push(a[i] as char)
        }
    }
    result
}

fn checksum(box_ids: &VecDeque<String>) -> u32 {
    let mut two_found_count: u32 = 0;
    let mut three_found_count: u32 = 0;

    for box_id in box_ids.iter() {
        let mut char_counts = HashMap::new();

        let mut two_found: bool = false;
        let mut three_found: bool = false;

        for character in box_id.as_bytes() {
            match char_counts.insert(character, 1) {
                None => {},
                Some(count) => {
                    char_counts.insert(character, count + 1);
                },
            };
        }

        for count in char_counts.values() {
            if two_found && three_found {
                break;
            }
            if *count == 2 {
                two_found = true;
            }
            if *count == 3 {
                three_found = true;
            }
        }

        if two_found {
            two_found_count += 1;
        }

        if three_found {
            three_found_count += 1;
        }
    }

    two_found_count * three_found_count
}

fn main() {
    let mut box_ids: VecDeque<String> = VecDeque::new();

    for line in io::stdin().lock().lines() {
        let box_id = line.unwrap();
        box_ids.push_back(box_id.clone());
    }

    let length = box_ids.len();
    let mut common_chars = String::new();

    'find_correct_boxes:
    for i in 0..length {
        for j in (i + 1)..length {
            let a = box_ids.get(i).unwrap();
            let b = box_ids.get(j).unwrap();

            if box_id_diff(a, b) == 1 {
                common_chars = common_bytes(a, b);
                break 'find_correct_boxes;
            }
        }
    }

    println!("Checksum: {}", checksum(&box_ids));
    println!("Common characters between strings: {}", common_chars);
}
