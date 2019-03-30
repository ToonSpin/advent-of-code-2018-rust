use std::io;
use std::io::prelude::*;

use std::cmp::{max, Ordering};
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

type Step = char;
type Deps = HashMap<Step, HashSet<Step>>;

#[derive(Debug, Copy, Clone, Ord, Eq)]
struct Worker {
    done: u32,
    step: Option<char>,
}

impl PartialOrd for Worker {
    fn partial_cmp(&self, other: &Worker) -> Option<Ordering> {
        if self.step == None && other.step == None {
            Some(Ordering::Equal)
        } else if self.step != None && other.step != None {
            Some(self.done.cmp(&other.done))
        } else if self.step == None {
            Some(Ordering::Less)
        } else {
            Some(Ordering::Greater)
        }
    }
}

impl PartialEq for Worker {
    fn eq(&self, other: &Worker) -> bool {
        self.done == other.done && self.step == other.step
    }
}

fn get_next_step<'a>(deps: &'a Deps) -> Option<&'a char> {
    let independents = deps.keys().filter(|k| deps.get(k).unwrap().is_empty());
    let mut independents: Vec<&Step> = independents.collect();
    independents.sort();
    match independents.first() {
        Some(c) => Some(*c),
        None => None,
    }
}

fn get_next_step_with_workers<'a>(deps: &'a Deps, workers: &'a [Worker]) -> Option<&'a char> {
    let independents = deps.keys().filter(|k| deps.get(k).unwrap().is_empty());
    let mut independents: HashSet<&Step> = independents.collect();

    for worker in workers {
        if let Some(s) = worker.step {
            independents.remove(&s);
        }
    }
    let mut independents = Vec::from_iter(independents.iter());
    independents.sort();
    match independents.first() {
        Some(c) => Some(*c),
        None => None,
    }
}

fn duration(step: &Step) -> u32 {
    60 + *step as u32 - b'A' as u32 + 1
}

fn main() -> io::Result<()> {
    let mut deps: Deps = HashMap::new();

    for line in io::stdin().lock().lines() {
        let line: Vec<u8> = line.unwrap().bytes().collect();

        let dependency = line[5] as Step;
        let dependent = line[36] as Step;

        deps.entry(dependency).or_insert(HashSet::new());
        let d = deps.entry(dependent).or_insert(HashSet::new());
        d.insert(dependency);
    }

    let mut ordering_deps = deps.clone();

    print!("The order in which the steps should be completed: ");

    loop {
        match get_next_step(&ordering_deps) {
            Some(next_step) => {
                let next_step = next_step.clone();
                print!("{}", next_step);

                for (_step, dependencies) in ordering_deps.iter_mut() {
                    dependencies.remove(&next_step);
                }
                ordering_deps.remove(&next_step);
            }
            None => {
                break;
            }
        }
    }

    println!("");

    let mut workers = [Worker {
        step: None,
        done: 0u32,
    }; 5];
    let mut current_timestamp = 0u32;
    let mut working_on = 0u32;
    loop {
        for mut worker in workers.iter_mut() {
            if worker.done <= current_timestamp && worker.step != None {
                for (_step, dependencies) in deps.iter_mut() {
                    dependencies.remove(&worker.step.unwrap());
                }
                deps.remove(&worker.step.unwrap());

                worker.step = None;
                working_on -= 1;
            }
        }
        workers.sort();

        match get_next_step_with_workers(&deps, &workers) {
            Some(next_step) => {
                let next_step = next_step.clone();
                workers[0] = Worker {
                    step: Some(next_step),
                    done: current_timestamp + duration(&next_step),
                };
                working_on += 1;
                workers.sort();
                current_timestamp = max(current_timestamp, workers[0].done);
            }
            None => {
                if working_on == 0 {
                    break;
                }
                for worker in workers.iter() {
                    if let Some(_s) = worker.step {
                        current_timestamp = worker.done;
                        break;
                    }
                }
            }
        }
    }

    println!(
        "Time needed for all workers to finish: {}",
        workers.last().unwrap().done
    );

    Ok(())
}
