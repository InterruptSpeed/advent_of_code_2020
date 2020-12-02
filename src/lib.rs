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

pub fn find_terms(inputs: Vec<u32>, sum: u32) -> (u32, u32) {
    // initialize a new BTree
    let mut inputs_btree = BTreeMap::new();

    // add each of the inputs to the btree
    for e in &inputs {
        inputs_btree.insert(e, e);
    }

    let mut x = 0;
    let mut y = 0;
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

pub fn solve_day_1a(filename: &str) -> u32 {
    let inputs = load_inputs(filename);

    let (x, y) = find_terms(inputs.to_vec(), 2020);
    let sum = x + y;
    let product = x * y;
    println!("In this list, the two entries that sum to {} are {} and {}. Multiplying them together produces {} * {} = {}, so the correct answer is {}.", sum, x, y, x, y, product, product);
    println!("{} * {} = {}", x, y, product); 
    product
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_terms_works() {
        // read inputs
        let inputs = vec![1721, 979, 366, 299, 675, 1456];
        // and sort them
        //inputs.sort();

        let (x, y) = find_terms(inputs.to_vec(), 2020);
        assert_eq!(x + y, 2020);
    }

    #[test]
    fn load_inputs_works() {
        let filename = "./data/day_1_input.txt";
        let inputs = load_inputs(filename);

        assert!(inputs.len() > 0);
    }

    #[test]
    fn solve_day_1a_works() {
        let filename = "./data/day_1_input.txt";
        let result = solve_day_1a(filename);
        assert!(result > 0);
    }
}