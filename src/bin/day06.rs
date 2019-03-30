use std::io;
use std::io::prelude::*;
use std::cmp::Ordering;
use std::cmp::min;
use std::cmp::max;

use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Clone, Hash, Eq)]
struct Point(i32, i32);

impl PartialEq for Point {
    fn eq(&self, other: &Point) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}

impl Point {
    fn dist(&self, x: i32, y: i32) -> i32 {
        (self.0 - x).abs() + (self.1 - y).abs()
    }

    fn left_turn(a: &Point, b: &Point, c: &Point) -> bool {
        (b.0 - a.0) * (c.1 - a.1) - (b.1 - a.1) * (c.0 - a.0) > 0
    }

    fn convex_hull(input:&Vec<Point>) -> HashSet<Point> {
        let mut input = input.clone();

        input.sort_by(|p, q| return if p.0 == q.0 { p.1.cmp(&q.1) } else { p.0.cmp(&q.0) });

        let origin_x = input[0].0;
        let origin_y = input[0].1;

        for i in 0..input.len() {
            input[i].0 -= origin_x;
            input[i].1 -= origin_y;
        }

       let compare_points = |a: &Point, b: &Point| -> Ordering {
            if (a.0 > 0 && b.0 > 0) || (a.0 < 0 && b.0 < 0) {
                return (a.1 * b.0).cmp(&(b.1 * a.0));
            } else {
                if a.1 == 0 {
                    return Ordering::Less;
                }
                if b.1 == 0 {
                    return Ordering::Greater;
                }
                return b.0.cmp(&a.0);
            }
        };

        input.sort_by(compare_points);

        let mut ch: Vec<Point> = Vec::new();
        let mut input_iter = input.into_iter();

        ch.push(input_iter.next().unwrap());
        ch.push(input_iter.next().unwrap());

        for p in input_iter {
            ch.push(p);
            while ch.len() >= 3 && !Point::left_turn(&ch[ch.len() - 3], &ch[ch.len() - 2], &ch[ch.len() - 1]) {
                ch.swap_remove(ch.len() - 2);
            }
        }

        ch.iter().map(|p| Point(p.0 + origin_x, p.1 + origin_y)).collect()
    }
}

fn main () -> io::Result<()> {
    let mut input:Vec<Point> = Vec::new();
    let mut areas: HashMap<Point,i32> = HashMap::new();

    let mut min_x: i32 = 10000;
    let mut max_x: i32 = 0;
    let mut min_y: i32 = 10000;
    let mut max_y: i32 = 0;

    for line in io::stdin().lock().lines() {
        let coords = line.unwrap();
        let mut coords = coords.split(',');

        let x: i32 = coords.next().unwrap().parse().unwrap();
        let y: i32 = coords.next().unwrap().trim_start().parse().unwrap();

        min_x = min(x, min_x);
        max_x = max(x, max_x);
        min_y = min(y, min_y);
        max_y = max(y, max_y);

        input.push(Point(x, y));
    }

    let convex_hull = Point::convex_hull(&input);

    let infinity = 2 * (max_y - min_y + max_x - min_y);
    let threshold = 10000;

    let mut near_distance_count = 0;
    for y in min_y..max_y {
        for x in min_x..max_x {
            let mut min_dist = infinity;
            let mut total_dist = 0;
            let mut min_dist_point: Option<Point> = None;
            let mut nonunique: bool = false;

            for p in input.iter() {
                let dist = p.dist(x, y);
                total_dist += dist;

                if dist == min_dist {
                    nonunique = true;
                }
                if dist < min_dist {
                    min_dist = dist;
                    min_dist_point = Some(p.clone());
                    nonunique = false;
                }
            }

            if total_dist < threshold {
                near_distance_count += 1;
            }

            if !nonunique {
                match min_dist_point {
                    Some(p) => {
                        if !convex_hull.contains(&p) {
                            let area = areas.entry(p).or_insert(0);
                            *area += 1;
                        }
                    },
                    None => {
                        panic!("No minimum distance found. Is input empty?");
                    },
                }
            }
        }
    }

    println!("Size of the largest area that isn't infinite: {}", areas.values().max().unwrap());
    println!("Size of region with total distance below {}: {}", threshold, near_distance_count);

    Ok(())
}
