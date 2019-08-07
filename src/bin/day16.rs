use std::io;
use std::io::prelude::*;

use std::iter::FromIterator;
use std::collections::{HashSet, HashMap};

extern crate nom;

use nom::{
    bytes::complete::tag,
    character::complete::{char as parse_char, digit1},
    combinator::{map_res, opt},
    multi::{many1, many_m_n, separated_list},
    sequence::{delimited, terminated},
    IResult,
};

type RegisterValue = u32;
type RegisterSet = [RegisterValue; 4];

struct DeviceState {
    registers: RegisterSet,
}

impl DeviceState {
    fn new(registers: RegisterSet) -> Self {
        DeviceState { registers }
    }

    fn get_all_operations() -> Vec<&'static str> {
        vec![
            "addr", "banr", "borr", "mulr", "setr",
            "addi", "bani", "bori", "muli", "seti",
            "eqir", "eqrr", "eqri",
            "gtir", "gtrr", "gtri",
        ]
    }

    fn operation(
        self: &mut Self,
        operation: &str,
        a: RegisterValue,
        b: RegisterValue,
        c: RegisterValue,
    ) {
        match operation {
            "addr" => { self.registers[c as usize] = self.registers[a as usize] + self.registers[b as usize]; }
            "banr" => { self.registers[c as usize] = self.registers[a as usize] & self.registers[b as usize]; }
            "borr" => { self.registers[c as usize] = self.registers[a as usize] | self.registers[b as usize]; }
            "mulr" => { self.registers[c as usize] = self.registers[a as usize] * self.registers[b as usize]; }
            "setr" => { self.registers[c as usize] = self.registers[a as usize]; }

            "addi" => { self.registers[c as usize] = self.registers[a as usize] + b; }
            "bani" => { self.registers[c as usize] = self.registers[a as usize] & b; }
            "bori" => { self.registers[c as usize] = self.registers[a as usize] | b; }
            "muli" => { self.registers[c as usize] = self.registers[a as usize] * b; }
            "seti" => { self.registers[c as usize] = a; }

            "eqir" => { self.registers[c as usize] = if a == self.registers[b as usize] { 1 } else { 0 }; }
            "eqrr" => { self.registers[c as usize] = if self.registers[a as usize] == self.registers[b as usize] { 1 } else { 0 }; }
            "eqri" => { self.registers[c as usize] = if self.registers[a as usize] == b { 1 } else { 0 }; }

            "gtir" => { self.registers[c as usize] = if a > self.registers[b as usize] { 1 } else { 0 }; }
            "gtrr" => { self.registers[c as usize] = if self.registers[a as usize] > self.registers[b as usize] { 1 } else { 0 }; }
            "gtri" => { self.registers[c as usize] = if self.registers[a as usize] > b { 1 } else { 0 }; }

            _ => unreachable!(),
        }
    }
}

fn parse_register_set_interior(input: &str) -> IResult<&str, Vec<&str>> {
    separated_list(tag(", "), digit1)(input)
}

fn check_register_set(input: Vec<&str>) -> Result<RegisterSet, ()> {
    if input.len() != 4 {
        Err(())
    } else {
        let mut s: RegisterSet = [0, 0, 0, 0];
        for (i, n) in input.iter().enumerate() {
            s[i] = n.parse().or(Err(()))?;
        }
        Ok(s)
    }
}

fn parse_register_set(input: &str) -> IResult<&str, RegisterSet> {
    delimited(
        parse_char('['),
        map_res(parse_register_set_interior, check_register_set),
        parse_char(']'),
    )(input)
}

fn parse_before(input: &str) -> IResult<&str, RegisterSet> {
    delimited(tag("Before: "), parse_register_set, parse_char('\n'))(input)
}

fn parse_sample_instruction(input: &str) -> IResult<&str, RegisterSet> {
    let parser = terminated(digit1, opt(parse_char(' ')));
    let parser = terminated(many_m_n(4, 4, parser), parse_char('\n'));
    map_res(parser, check_register_set)(input)
}

fn parse_after(input: &str) -> IResult<&str, RegisterSet> {
    delimited(tag("After:  "), parse_register_set, tag("\n\n"))(input)
}

struct TestSample {
    before: RegisterSet,
    after: RegisterSet,
    sample_instruction: RegisterSet,
}

fn parse_test_sample(input: &str) -> IResult<&str, TestSample> {
    let (input, before) = parse_before(input)?;
    let (input, sample_instruction) = parse_sample_instruction(input)?;
    let (rest, after) = parse_after(input)?;
    Ok((
        rest,
        TestSample {
            before,
            after,
            sample_instruction,
        },
    ))
}

fn parse_test_sample_list(input: &str) -> IResult<&str, Vec<TestSample>> {
    terminated(many1(parse_test_sample), many1(parse_char('\n')))(input)
}

fn parse_instruction_list(input: &str) -> IResult<&str, Vec<RegisterSet>> {
    let register_set_parser = map_res(separated_list(parse_char(' '), digit1), check_register_set);
    let register_set_parser = terminated(register_set_parser, parse_char('\n'));
    many1(register_set_parser)(input)
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    let input = &input[..];

    let (rest, test_samples) = parse_test_sample_list(input).unwrap();
    let (_rest, instructions) = parse_instruction_list(rest).unwrap();

    let mut part1_match = 0;
    let mut possible_instructions: HashMap<RegisterValue, HashSet<&str>> = HashMap::new();
    let mut definite_instructions: HashMap<RegisterValue, &str> = HashMap::new();

    for sample in test_samples {
        let mut count_part1_match = 0;
        for operation in DeviceState::get_all_operations() {
            let mut state = DeviceState::new(sample.before);
            state.operation(
                operation,
                sample.sample_instruction[1],
                sample.sample_instruction[2],
                sample.sample_instruction[3]
            );
            if state.registers == sample.after {
                count_part1_match += 1;
            } else {
                let instr = sample.sample_instruction[0];
                let possible_for_opcode = possible_instructions.entry(instr).or_insert(HashSet::from_iter(DeviceState::get_all_operations()));

                if possible_for_opcode.contains(operation) {
                    possible_for_opcode.remove(operation);
                }
            }
        }

        if count_part1_match >= 3 {
            part1_match += 1;
        }
    }

    let mut done = false;
    while !done {
        let mut instruction = 0;
        let mut opcode = "";
        done = true;
        for (k, v) in possible_instructions.iter() {
            if v.len() == 1 {
                done = false;
                instruction = *k;
                opcode = v.iter().next().unwrap();
                definite_instructions.insert(*k, opcode);
                break;
            }
        }

        let mut new_possible_instructions: HashMap<RegisterValue, HashSet<&str>> = HashMap::new();
        for (k, v) in possible_instructions.iter() {
            if *k == instruction {
                continue;
            }
            let mut s = HashSet::new();
            for o in v {
                if *o != opcode {
                    s.insert(*o);
                }
            }
            new_possible_instructions.insert(*k, s);
        }
        possible_instructions = new_possible_instructions;
    }

    println!("Number of samples that match at least three opcodes: {}", part1_match);

    let mut state = DeviceState::new([0, 0, 0, 0]);
    for i in instructions {
        let opcode = definite_instructions.get(&i[0]).unwrap();
        state.operation(opcode, i[1], i[2], i[3]);
    }

    println!("The value of register 0 after the test program: {}", state.registers[0]);

    Ok(())
}
