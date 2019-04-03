extern crate regex;
use regex::{Captures, Regex};

use std::io;
use std::io::prelude::*;

use std::cmp::{max, min};

struct Point {
    x: i64,
    y: i64,
    vx: i64,
    vy: i64,
}

impl Point {
    fn from_caps(caps: Captures) -> Point {
        let x = caps[1].parse().unwrap();
        let y = caps[2].parse().unwrap();
        let vx = caps[3].parse().unwrap();
        let vy = caps[4].parse().unwrap();
        Point { x, y, vx, vy }
    }
}

fn print_points(points: &Vec<Point>) -> () {
    let mut min_x = 100000i64;
    let mut max_x = -100000i64;
    let mut min_y = 100000i64;
    let mut max_y = -100000i64;

    for p in points.iter() {
        min_x = min(min_x, p.x);
        min_y = min(min_y, p.y);
        max_x = max(max_x, p.x);
        max_y = max(max_y, p.y);
    }

    let mut matrix = Vec::new();
    for _y in min_y..=max_y {
        let mut row = Vec::new();
        for _x in min_x..=max_x {
            row.push(' ');
        }
        matrix.push(row);
    }

    for p in points.iter() {
        matrix[(p.y - min_y) as usize][(p.x - min_x) as usize] = '#';
    }

    for line in matrix.iter() {
        for c in line.iter() {
            print!("{}", c);
        }
        println!("");
    }
}

fn main() -> io::Result<()> {
    let mut input = Vec::new();

    let re = Regex::new(
        r"^[^0-9-]+(-?[0-9]+)[^0-9-]+(-?[0-9]+)[^0-9-]+(-?[0-9]+)[^0-9-]+(-?[0-9]+)[^0-9-]+$",
    )
    .unwrap();
    for line in io::stdin().lock().lines() {
        if let Some(caps) = re.captures(line.unwrap().as_str()) {
            input.push(Point::from_caps(caps));
        }
    }

    let mut time_elapsed = 0u32;

    loop {
        time_elapsed += 1;

        let mut min_y = 100000i64;
        let mut max_y = -100000i64;

        for p in input.iter_mut() {
            p.x += p.vx;
            p.y += p.vy;

            min_y = min(min_y, p.y);
            max_y = max(max_y, p.y);
        }

        if max_y - min_y < 10 {
            break;
        }
    }

    print_points(&input);
    println!("Message appears after {} seconds.", time_elapsed);

    Ok(())
}
