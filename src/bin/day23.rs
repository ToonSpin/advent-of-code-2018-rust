use std::io;
use std::io::prelude::*;

use std::cmp::Ordering;
use std::collections::BinaryHeap;

extern crate nom;
use nom::{
    bytes::complete::tag,
    character::complete::{char as parse_char, digit1},
    combinator::{all_consuming, map_res, opt, recognize},
    sequence::{pair, preceded},
    IResult,
};

#[derive(Debug)]
struct Nanobot {
    x: i64,
    y: i64,
    z: i64,
    r: i64,
}

impl Nanobot {
    fn get_max_coordinate(&self) -> i64 {
        let max = |a, b| if a > b { a } else { b };
        let coord = max(self.x.abs(), self.y.abs());
        max(coord, self.z.abs())
    }
}

fn parse_i64(input: &str) -> IResult<&str, i64> {
    let parser = pair(opt(parse_char('-')), digit1);
    map_res(recognize(parser), |s: &str| s.parse::<i64>())(input)
}

fn parse_nanobot(input: &str) -> IResult<&str, Nanobot> {
    let (rest, x) = preceded(tag("pos=<"), parse_i64)(input)?;
    let (rest, y) = preceded(parse_char(','), parse_i64)(rest)?;
    let (rest, z) = preceded(parse_char(','), parse_i64)(rest)?;
    let (rest, r) = all_consuming(preceded(tag(">, r="), parse_i64))(rest)?;
    Ok((rest, Nanobot {x, y, z, r}))
}

#[derive(Debug, PartialEq, Eq)]
struct SearchSquare {
    o_x: i64,
    o_y: i64,
    o_z: i64,
    size: u64,
    num_in_range: u64
}

fn range_dist(x: i64, lo: i64, hi: i64) -> i64 {
    if x < lo {
        lo - x
    } else if x > hi {
        x - hi
    } else {
        0
    }
}

impl SearchSquare {
    fn distance_from_origin(&self) -> i64 {
        let mut d = 0;
        d += range_dist(0, self.o_x, self.o_x + self.size as i64);
        d += range_dist(0, self.o_y, self.o_y + self.size as i64);
        d += range_dist(0, self.o_z, self.o_z + self.size as i64);
        d
    }

    fn in_range(&self, bot: &Nanobot) -> bool {
        let mut d = 0;
        d += range_dist(bot.x, self.o_x, self.o_x + self.size as i64);
        d += range_dist(bot.y, self.o_y, self.o_y + self.size as i64);
        d += range_dist(bot.z, self.o_z, self.o_z + self.size as i64);
        d <= bot.r
    }

    fn new(x: i64, y: i64, z: i64, size: u64, bots: &Vec<Nanobot>) -> SearchSquare {
        let mut square = SearchSquare {
            o_x: x,
            o_y: y,
            o_z: z,
            size,
            num_in_range: 0
        };
        square.num_in_range = bots.iter().filter(|b| square.in_range(&b)).count() as u64;
        square
    }

    fn split(self, bots: &Vec<Nanobot>) -> Vec<SearchSquare> {
        let mut v = Vec::new();
        let size = self.size / 2;
        for p in 0..=1 {
            for q in 0..=1 {
                for r in 0..=1 {
                    let o_x = self.o_x + p * size as i64;
                    let o_y = self.o_y + q * size as i64;
                    let o_z = self.o_z + r * size as i64;
                    v.push(SearchSquare::new(o_x, o_y, o_z, size, bots));
                }
            }
        }
        v
    }
}

impl Ord for SearchSquare {
    fn cmp(&self, other: &Self) -> Ordering {
        self.num_in_range.cmp(&other.num_in_range)
            .then(self.distance_from_origin().cmp(&other.distance_from_origin()).reverse())
            .then(self.size.cmp(&other.size).reverse())
    }
}

impl PartialOrd for SearchSquare {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

fn main() -> io::Result<()> {
    let mut input = Vec::new();
    for line in io::stdin().lock().lines() {
        let line = &line.unwrap();
        let tuple = parse_nanobot(line).unwrap();
        input.push(tuple.1);
    }
    let mut max_coordinate = 0;
    for bot in input.iter() {
        let c = bot.get_max_coordinate();
        if c > max_coordinate {
            max_coordinate = c;
        }
    }
    let mut size = 1i64;
    while size < max_coordinate {
        size *= 2;
    }

    let square = SearchSquare::new(-size, -size, -size, (size * 2) as u64, &input);
    let mut heap = BinaryHeap::new();
    heap.push(square);

    loop {
        let square = heap.pop().unwrap();
        if square.size == 0 {
            println!("Shortest Manhattan distance among points in range of most nanobots: {}", square.distance_from_origin());
            break;
        }
        let split_squares = square.split(&input);
        for s in split_squares {
            heap.push(s);
        }
    }

    Ok(())
}
