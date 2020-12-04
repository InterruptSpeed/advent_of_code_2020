//! advent of code 2020

use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn load_inputs(filename: &str) -> Vec<u32> {
    let mut inputs = Vec::new();

    // no file exists checking for now
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap(); // Ignore errors
        let trimmed = line.trim();
        match trimmed.parse::<u32>() {
            Ok(i) => inputs.push(i),
            Err(..) => continue
        };
    }
    
    inputs
}

pub fn find_two_terms(inputs: Vec<u32>, sum: u32) -> (u32, u32) {
    let mut x = 0;
    let mut y = 0;

    // initialize a new BTree
    let mut inputs_btree = BTreeMap::new();

    // add each of the inputs to the btree
    for e in &inputs {
        inputs_btree.insert(e, e);
    }

    // iterate over each input
    for target_x in &inputs {
        let target_y = sum - target_x;
        if inputs_btree.contains_key(&target_y) {
            x = *target_x;
            y = target_y;
            break
        }
    }

    (x, y)
}

pub fn find_three_terms(inputs: Vec<u32>, sum: u32) -> (u32, u32, u32) {
    let mut x = 0;
    let mut y = 0;
    let mut z = 0;

    for i in 0..(inputs.len() - 2) {
        for j in 0..(inputs.len() - 1) {
            for k in 0..(inputs.len()) {
                if (inputs[i] + inputs[j] + inputs[k]) == sum {
                    x = inputs[i];
                    y = inputs[j];
                    z = inputs[k];
                }
            }
        }
    }

    (x, y, z)
}

pub fn solve_day_1a(filename: &str) -> u32 {
    let inputs = load_inputs(filename);

    let (x, y) = find_two_terms(inputs.to_vec(), 2020);
    let product = x * y;
    product
}

pub fn solve_day_1b(filename: &str) -> u32 {
    let inputs = load_inputs(filename);

    let (x, y, z) = find_three_terms(inputs.to_vec(), 2020);
    let product = x * y * z;
    product
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_two_terms_works() {
        // read inputs
        let inputs = vec![1721, 979, 366, 299, 675, 1456];
        // and sort them

        let (x, y) = find_two_terms(inputs.to_vec(), 2020);
        assert_eq!(x + y, 2020);
    }

    #[test]
    fn load_inputs_works() {
        let filename = "./data/day_1_input.txt";
        let inputs = load_inputs(filename);

        assert!(inputs.len() > 0);
    }

    #[test]
    fn find_three_terms_works() {
        let inputs = vec![1721, 979, 366, 299, 675, 1456];

        let (x, y, z) = find_three_terms(inputs.to_vec(), 2020);
        assert_eq!(x + y + z, 2020);
    }
    
}