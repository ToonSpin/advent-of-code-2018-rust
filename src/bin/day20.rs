use std::io;
use std::io::prelude::*;

extern crate nom;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char as parse_char, one_of},
    combinator::{map, peek, recognize},
    multi::{many1, separated_list},
    sequence::{delimited},
    IResult,
};

use std::collections::HashMap;

#[derive(Debug)]
enum RouteSpec<'a> {
    Route(&'a str),
    Branches(Vec<RouteSpec<'a>>),
    Sequence(Vec<RouteSpec<'a>>),
}

fn parse_empty_route(input: &str) -> IResult<&str, RouteSpec> {
    map(recognize(peek(one_of("|)"))), |s| RouteSpec::Route(s))(input)
}

fn parse_route(input: &str) -> IResult<&str, RouteSpec> {
    map(recognize(many1(one_of("NESW"))), |s| RouteSpec::Route(s))(input)
}

fn parse_branches(input: &str) -> IResult<&str, RouteSpec> {
    let parser = delimited(parse_char('('), separated_list(parse_char('|'), alt((parse_routespec, parse_empty_route))), parse_char(')'));
    map(parser, |v| RouteSpec::Branches(v))(input)
}

fn parse_routespec(input: &str) -> IResult<&str, RouteSpec> {
    let parser = many1(alt((parse_branches, parse_route)));
    map(parser, |v| RouteSpec::Sequence(v))(input)
}

fn parse_regex(input: &str) -> IResult<&str, RouteSpec> {
    delimited(parse_char('^'), parse_routespec, tag("$\n"))(input)
}

type Distance = u32;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Coords(i32, i32);

impl Coords {
    fn north(&mut self) -> () {
        self.1 -= 1;
    }
    fn east(&mut self) -> () {
        self.0 += 1;
    }
    fn south(&mut self) -> () {
        self.1 += 1;
    }
    fn west(&mut self) -> () {
        self.0 -= 1;
    }
}

fn compute_distances(r: &RouteSpec, offset: &mut Coords, mut distances: HashMap<Coords, Distance>) -> HashMap<Coords, Distance> {
    match r {
        RouteSpec::Branches(v) => {
            for branch in v.iter() {
                let mut _offset = offset.clone();
                distances = compute_distances(branch, &mut _offset, distances);
            }
        },
        RouteSpec::Sequence(v) => {
            for branch in v.iter() {
                distances = compute_distances(branch, offset, distances);
            }
        },
        RouteSpec::Route(s) => {
            let mut current_distance = *distances.entry(*offset).or_insert(0);
            for c in s.chars() {
                match c {
                    'N' => offset.north(),
                    'E' => offset.east(),
                    'S' => offset.south(),
                    'W' => offset.west(),
                    _ => unreachable!(),
                };
                current_distance += 1;
                match distances.get(&offset) {
                    Some(d) => { if current_distance < *d { distances.insert(*offset, current_distance); } }
                    None => { distances.insert(*offset, current_distance); }
                }
            }
        }
    }
    distances
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    let input = &input[..];

    let (_rest, route) = parse_regex(input).unwrap();

    let mut offset = Coords(0, 0);
    let distances = compute_distances(&route, &mut offset, HashMap::new());

    let mut max_distance = 0u32;
    let mut far_room_count = 0;
    for d in distances.values().into_iter() {
        if *d > max_distance {
            max_distance = *d;
        }
        if *d >= 1000 {
            far_room_count += 1;
        }
    }

    println!("The most distant room is {} steps away.", max_distance);
    println!("There are {} rooms that are at least 1000 steps away.", far_room_count);

    Ok(())
}
