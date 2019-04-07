use std::io;
use std::io::prelude::*;

use std::cmp::Ord;
use std::cmp::Ordering;

use std::collections::HashMap;

#[derive(Eq)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl PartialEq for Direction {
    fn eq(&self, other: &Direction) -> bool {
        self == other
    }
}

#[derive(Eq)]
enum TurnDirection {
    Left,
    Straight,
    Right,
}

impl PartialEq for TurnDirection {
    fn eq(&self, other: &TurnDirection) -> bool {
        self == other
    }
}


#[derive(Eq)]
struct Cart {
    pos: (u32, u32),
    dir: Direction,
    next_turndir: TurnDirection,
}

impl Cart {
    fn new(pos: (u32, u32), dir: Direction) -> Cart {
        Cart {
            pos,
            dir,
            next_turndir: TurnDirection::Left,
        }
    }

    fn turn_left(&mut self) -> () {
        self.dir = match self.dir {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        };
    }

    fn turn_right(&mut self) -> () {
        self.dir = match self.dir {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        };
    }

    fn move_forward(&mut self) -> () {
        match self.dir {
            Direction::North => { self.pos.1 -= 1; },
            Direction::East  => { self.pos.0 += 1; },
            Direction::South => { self.pos.1 += 1; },
            Direction::West  => { self.pos.0 -= 1; },
        }
    }

    fn process_crossing(&mut self) -> () {
        match self.next_turndir {
            TurnDirection::Left => {
                self.turn_left();
                self.next_turndir = TurnDirection::Straight;
            }
            TurnDirection::Straight => {
                self.next_turndir = TurnDirection::Right;
            }
            TurnDirection::Right => {
                self.turn_right();
                self.next_turndir = TurnDirection::Left;
            }
        }
    }
}

impl PartialOrd for Cart {
    fn partial_cmp(&self, other: &Cart) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Cart {
    fn eq(&self, other: &Cart) -> bool {
        self.pos.0 == other.pos.0 && self.pos.1 == other.pos.1
    }
}

impl Ord for Cart {
    fn cmp(&self, other: &Cart) -> Ordering {
        match self.pos.1.cmp(&other.pos.1) {
            Ordering::Equal => {
                self.pos.0.cmp(&other.pos.0)
            },
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
        }
    }
}

struct Track {
    track: Vec<Vec<u8>>,
    carts: Vec<Cart>,
    current_cart_index: usize,
}

impl Track {
    fn new(lines: std::io::Lines<std::io::StdinLock>) -> Track {
        let mut carts = Vec::new();
        let mut track = Vec::new();

        for (y, line) in lines.enumerate() {
            let y = y as u32;
            let mut track_row: Vec<u8> = Vec::new();
            let mut x = 0;
            for b in line.unwrap().into_bytes() {
                match b {
                    b'^' => {
                        let cart = Cart::new((x, y), Direction::North);
                        carts.push(cart);
                        track_row.push(b'|');
                    }
                    b'v' => {
                        let cart = Cart::new((x, y), Direction::South);
                        carts.push(cart);
                        track_row.push(b'|');
                    }
                    b'>' => {
                        let cart = Cart::new((x, y), Direction::East);
                        carts.push(cart);
                        track_row.push(b'-');
                    }
                    b'<' => {
                        let cart = Cart::new((x, y), Direction::West);
                        carts.push(cart);
                        track_row.push(b'-');
                    }
                    _ => {
                        track_row.push(b);
                    }
                }
                x += 1;
            }
            track.push(track_row);
        }

        Track {
            carts,
            track,
            current_cart_index: 0
        }
    }

    fn end_of_tick(&mut self) -> bool {
        self.current_cart_index == self.carts.len()
    }

    fn find_and_process_collision(&mut self) -> Option<(u32, u32)> {
        let mut positions_found: HashMap<(u32, u32), usize> = HashMap::new();
        let mut pos_found: Option<(u32, u32)> = None;

        for (i, cart) in self.carts.iter().enumerate() {
            if let Some(j) = positions_found.insert(cart.pos, i) {
                let a;
                let b;

                if i > j {
                    a = i;
                    b = j;
                } else {
                    a = j;
                    b = i;
                }

                pos_found = Some(cart.pos);
                if self.current_cart_index > a {
                    self.current_cart_index -= 1;
                }
                if self.current_cart_index > b {
                    self.current_cart_index -= 1;
                }

                self.carts.remove(a);
                self.carts.remove(b);

                break;
            }
        }

        pos_found
    }

    fn iterate(&mut self) -> () {
        if self.current_cart_index == self.carts.len() {
            self.carts.sort();
            self.current_cart_index = 0;
        }

        let cart = &mut self.carts[self.current_cart_index];

        match self.track[cart.pos.1 as usize][cart.pos.0 as usize] as char {
            '+' => {
                cart.process_crossing();
            },
            '/' => {
                match cart.dir {
                    Direction::West | Direction::East => {
                        cart.turn_left();
                    },
                    Direction::North | Direction::South => {
                        cart.turn_right();
                    },
                }
            },
            '\\' => {
                match cart.dir {
                    Direction::West | Direction::East => {
                        cart.turn_right();
                    },
                    Direction::North | Direction::South => {
                        cart.turn_left();
                    },
                }
            },
            _ => {}, 
        }
        cart.move_forward();

        self.current_cart_index += 1;
    }
}

fn main() -> io::Result<()> {
    let mut track = Track::new(io::stdin().lock().lines());

    let mut found = false;
    loop {
        track.iterate();
        if let Some(pos) = track.find_and_process_collision() {
            if !found {
                println!("Position of the first collision: {},{}", pos.0, pos.1);
                found = true;
            }
        }
        if track.end_of_tick() {
            if track.carts.len() == 1 {
                let cart = track.carts.pop().unwrap();
                println!("Position of the last cart at the end of the tick: {},{}", cart.pos.0, cart.pos.1);
                break;
            }
        }
    }

    Ok(())
}
