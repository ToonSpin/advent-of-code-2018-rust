use std::io;
use std::io::prelude::*;

fn num_to_vec_u8(num: usize) -> Vec<u8> {
    let mut v = Vec::new();
    let mut num = num;
    while num > 0 {
        v.push((num % 10) as u8);
        num /= 10;
    }
    v.reverse();
    v
}

fn check_for_match(recipes: &Vec<u8>, numv: &Vec<u8>) -> bool {
    if recipes.len() < numv.len() {
        false
    } else {
        &recipes[recipes.len() - numv.len()..] == &numv[..]
    }
}

fn main() -> io::Result<()> {
    let mut num = 0usize;
    for line in io::stdin().lock().lines() {
        num = line.unwrap().parse().unwrap();
    }

    let mut recipes: Vec<u8> = Vec::with_capacity(33554432);
    recipes.push(3);
    recipes.push(7);

    let mut a: usize = 0;
    let mut b: usize = 1;

    let mut i: usize = 0;
    let mut found = 0usize;

    let numv = num_to_vec_u8(num);
    let lastrecipe: u8 = *numv.last().unwrap();

    while i < num + 10 || found == 0 {
        let newrecipe = recipes[a] + recipes[b];

        if newrecipe > 9 {
            let next_recipe = newrecipe / 10;
            recipes.push(next_recipe);
            if found == 0 && next_recipe == lastrecipe {
                if check_for_match(&recipes, &numv) {
                    found = recipes.len() - numv.len();
                }
            }
        }

        let next_recipe = newrecipe % 10;
        recipes.push(next_recipe);
        if found == 0 && next_recipe == lastrecipe {
            if check_for_match(&recipes, &numv) {
                found = recipes.len() - numv.len();
            }
        }

        a = (a + recipes[a] as usize + 1) % recipes.len();
        b = (b + recipes[b] as usize + 1) % recipes.len();

        i += 1;
    }

    print!("The next ten recipes: ");
    for recipe in num..num + 10 {
        print!("{}", recipes[recipe]);
    }
    println!("");

    println!("The sequence appears after: {}", found);

    Ok(())
}
