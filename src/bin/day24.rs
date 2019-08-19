use std::io;
use std::io::prelude::*;

extern crate nom;

use nom::{
    branch::alt,
    bytes::complete::{is_not, tag},
    character::complete::{alpha1, char as parse_char, digit1},
    combinator::{map, map_res, opt},
    IResult,
    multi::separated_list,
    sequence::{delimited, separated_pair, terminated},
};

use std::cmp::Ordering;

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
enum Faction {
    ImmuneSystem,
    Infection
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Group<'a> {
    id: usize,
    faction: Faction,
    num_units: u64,
    hit_points: u64,
    weaknesses: Vec<&'a str>,
    immunities: Vec<&'a str>,
    attack_damage: u64,
    attack_damage_type: &'a str,
    initiative: u64,
}

impl<'a> Group<'a> {
    fn effective_power(&self) -> u64 {
        self.num_units * self.attack_damage
    }

    fn incur_attack(&mut self, attacker: &Self) -> u64 {
        let mut damage = self.damage_dealt_by(attacker) / self.hit_points;

        if damage > self.num_units {
            damage = self.num_units;
        }

        self.num_units -= damage;

        return damage;
    }

    fn damage_dealt_by(&self, attacker: &Self) -> u64 {
        attacker.damage_dealt_to(self)
    }

    fn damage_dealt_to(&self, defender: &Self) -> u64 {
        if defender.immunities.contains(&self.attack_damage_type) {
            0
        } else if defender.weaknesses.contains(&self.attack_damage_type) {
            2 * self.effective_power()
        } else {
            self.effective_power()
        }
    }
}

fn parse_weakness_or_immunity(input: &str) -> IResult<&str, (&str, Vec<&str>)> {
    separated_pair(alt((tag("immune"), tag("weak"))), tag(" to "), separated_list(tag(", "), alpha1))(input)
}

fn parse_weaknesses_and_immunities(input: &str) -> IResult<&str, Vec<(&str, Vec<&str>)>> {
    separated_list(tag("; "), parse_weakness_or_immunity)(input)
}

fn parse_group(input: &str) -> IResult<&str, Group> {
    let mut weaknesses = Vec::new();
    let mut immunities = Vec::new();

    let (rest, num_units) = map_res(terminated(digit1, tag(" units each with ")), str::parse::<u64>)(input)?;
    let (rest, hit_points) = map_res(terminated(digit1, tag(" hit points ")), str::parse::<u64>)(rest)?;
    let (rest, weaknesses_immunities) = opt(delimited(parse_char('('), is_not(")"), tag(") ")))(rest)?;

    if let Some(weaknesses_immunities) = weaknesses_immunities {
        let (_rest, v) = parse_weaknesses_and_immunities(weaknesses_immunities)?;
        for (value_type, damage_types) in v {
            match value_type {
                "immune" => { immunities = damage_types; }
                "weak" => { weaknesses = damage_types; }
                _ => { unreachable!(); }
            }
        }
    }

    let (rest, attack_damage) = map_res(delimited(tag("with an attack that does "), digit1, tag(" ")), str::parse::<u64>)(rest)?;
    let (rest, attack_damage_type) = terminated(alpha1, tag(" damage at initiative "))(rest)?;
    let (rest, initiative) = map_res(digit1, str::parse::<u64>)(rest)?;

    Ok((rest, Group {
        id: 0,
        faction: Faction::Infection, // gets overwritten with correct value later
        num_units,
        hit_points,
        weaknesses,
        immunities,
        attack_damage,
        attack_damage_type,
        initiative,
    }))
}

fn parse_immune_system(input: &str) -> IResult<&str, Vec<Group>> {
    let parser = map(parse_group, |mut g| {g.faction = Faction::ImmuneSystem; g});
    delimited(tag("Immune System:\n"), separated_list(parse_char('\n'), parser), tag("\n\n"))(input)
}

fn parse_infection(input: &str) -> IResult<&str, Vec<Group>> {
    let parser = map(parse_group, |mut g| {g.faction = Faction::Infection; g});
    delimited(tag("Infection:\n"), separated_list(parse_char('\n'), parser), tag("\n"))(input)
}

fn compare_for_target_selection(a: &Group, b: &Group) -> Ordering {
    a.effective_power().cmp(&b.effective_power()).then(a.initiative.cmp(&b.initiative)).reverse()
}

fn get_groups_from_input(input: &str) -> Vec<Group> {
    let (rest, mut groups) = parse_immune_system(input).unwrap();
    let (_rest, mut infection) = parse_infection(rest).unwrap();
    groups.append(&mut infection);

    let mut last_id = 0;
    for g in groups.iter_mut() {
        g.id = last_id;
        last_id += 1;
    }

    groups
}

fn find_group_by_id(groups: &Vec<Group>, id: usize) -> usize {
    for (i, g) in groups.iter().enumerate() {
        if g.id == id {
            return i;
        }
    }
    unreachable!();
}

fn get_group_counts(groups: &Vec<Group>) -> (u64, u64) {
    let mut found_immune_system = 0;
    let mut found_infection = 0;

    for g in groups.iter() {
        match g.faction {
            Faction::ImmuneSystem => {
                if g.num_units > 0 {
                    found_immune_system += g.num_units;
                }
            },
            Faction::Infection => {
                if g.num_units > 0 {
                    found_infection += g.num_units;
                }
            },
        }
    }
    (found_immune_system, found_infection)
}

fn is_fight_done(groups: &Vec<Group>) -> bool {
    let (found_immune_system, found_infection) = get_group_counts(groups);
    found_immune_system == 0 || found_infection == 0
}

fn units_left_after_battle(mut groups: Vec<Group>, boost: u64) -> (u64, u64) {
    for g in groups.iter_mut().filter(|g| g.faction == Faction::ImmuneSystem) {
        g.attack_damage += boost;
    }
    while !is_fight_done(&groups) {
        let mut targeted_by: Vec<Option<usize>> = vec![None; groups.len()];
        let mut target: Vec<Option<usize>> = vec![None; groups.len()];

        // ---------------------------------------------------- TARGET SELECTION
        groups.sort_unstable_by(compare_for_target_selection);
        let target_selection_order: Vec<usize> = groups.iter().map(|g| g.id).collect();

        for id in target_selection_order.iter() {
            let attacker = &groups[find_group_by_id(&groups, *id)].clone();
            let cmp = |a: &Group, b: &Group| {
                a.damage_dealt_by(&attacker).cmp(&b.damage_dealt_by(&attacker))
                    .then(a.effective_power().cmp(&b.effective_power()))
                    .then(a.initiative.cmp(&b.initiative))
                    .reverse()
            };
            groups.sort_unstable_by(cmp);

            for g in groups.iter().filter(|g| g.num_units > 0 && g.faction != attacker.faction) {
                if g.damage_dealt_by(attacker) == 0 {
                    continue;
                }
                if let None = targeted_by[g.id] {
                    targeted_by[g.id] = Some(attacker.id);
                    target[attacker.id] = Some(g.id);
                    break;
                } else {
                }
            }
        }

        // -------------------------------------------------------- ATTACK PHASE
        groups.sort_unstable_by(|a, b| a.initiative.cmp(&b.initiative).reverse());
        let attack_order: Vec<usize> = groups.iter().map(|g| g.id).collect();
        let mut kills = 0;
        for attacker_id in attack_order.iter() {
            let attacker = groups[find_group_by_id(&groups, *attacker_id)].clone();
            if attacker.num_units == 0 {
                continue;
            }
            match target[attacker.id] {
                Some(defender_id) => {
                    let defender_index = find_group_by_id(&groups, defender_id);
                    let defender = &mut groups[defender_index];
                    kills += defender.incur_attack(&attacker);
                },
                None => {
                    continue
                },
            }
        }
        if kills == 0 {
            break; // break stalemate
        }
    }

    get_group_counts(&groups)
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    let input = &input[..];
    let groups = get_groups_from_input(input);

    let (_found_immune_system, found_infection) = units_left_after_battle(groups.clone(), 0);
    println!("Number of units left after pre-boost battle: {}", found_infection);

    let mut boost = 0;
    loop {
        let (found_immune_system, found_infection) = units_left_after_battle(groups.clone(), boost);
        if found_infection == 0 {
            println!("Number of units left after post-boost battle: {}", found_immune_system);
            break;
        }
        boost += 1;
    }

    Ok(())
}
