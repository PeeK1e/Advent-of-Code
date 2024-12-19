use std::fs;

use solution::solve::{solve_t1, solve_t2};

fn main() {
    let input = fs::read_to_string("./input").unwrap();
    println!("solution 1: {}", solve_t1(&input,71, 71, 1024).unwrap());
    let ele = solve_t2(&input,71,71,1024).unwrap();
    println!("solution 2: {},{}", ele.0, ele.1);
}
