use std::io;
use std::io::prelude::*;

extern crate regex;
use regex::Regex;

use std::collections::VecDeque;

struct Claim {
    id: u32,
    x: u32,
    y: u32,
    w: u32,
    h: u32,
}

fn number_of_claims(x: u32, y: u32, claims: &VecDeque<Claim>) -> u32 {
    let mut count = 0;
    for claim in claims.iter() {
        if y >= claim.y && y < claim.y + claim.h {
            if x >= claim.x && x < claim.x + claim.w {
                count += 1;
            }
        }
    }
    count
}

fn main() {
    let mut claims = VecDeque::new();
    let mut cloth = [[0u32; 1000]; 1000];

    let re = Regex::new(r"^#(\d+) @ (\d+),(\d+): (\d+)x(\d+)$").unwrap();
    for line in io::stdin().lock().lines() {
        if let Some(caps) = re.captures(line.unwrap().as_str()) {
            claims.push_back(Claim {
                id: caps[1].parse().unwrap(),
                x: caps[2].parse().unwrap(),
                y: caps[3].parse().unwrap(),
                w: caps[4].parse().unwrap(),
                h: caps[5].parse().unwrap(),
            });
        }
    }

    for claim in claims.iter() {
        for y in claim.y..(claim.y + claim.h) {
            for x in claim.x..(claim.x + claim.w) {
                cloth[y as usize][x as usize] += 1;
            }
        }
    }

    let mut count: u32 = 0;
    for y in 0..1000 {
        for x in 0..1000 {
            if cloth[y as usize][x as usize] > 1 {
                count += 1;
            }
        }
    }

    println!("Number of squares with more than one claim: {}", count);

    'claim_check: for claim in claims.iter() {
        for y in claim.y..(claim.y + claim.h) {
            for x in claim.x..(claim.x + claim.w) {
                if number_of_claims(x, y, &claims) > 1 {
                    continue 'claim_check;
                }
            }
        }

        println!("ID of claim with no overlapping other claims: {}", claim.id);
        break;
    }
}
