use std::io;
use std::io::prelude::*;

use std::collections::HashMap;

fn boxes_match(a: &String, b: &String) -> bool {
    let a = a.as_bytes();
    let b = b.as_bytes();

    let mut found = false;

    for i in 0..a.len() {
        if a[i] != b[i] {
            if found {
                return false;
            }
            found = true;
        }
    }

    true
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

fn checksum(box_ids: &Vec<String>) -> u32 {
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
    let mut box_ids: Vec<String> = Vec::new();

    for line in io::stdin().lock().lines() {
        let box_id = line.unwrap();
        box_ids.push(box_id);
    }

    let length = box_ids.len();
    let mut common_chars = String::new();

    'find_correct_boxes:
    for (i, a) in box_ids.iter().enumerate() {
        for j in (i + 1)..length {
            let b = box_ids.get(j).unwrap();

            if boxes_match(a, b) {
                common_chars = common_bytes(a, b);
                break 'find_correct_boxes;
            }
        }
    }

    println!("Checksum: {}", checksum(&box_ids));
    println!("Common characters between strings: {}", common_chars);
}
