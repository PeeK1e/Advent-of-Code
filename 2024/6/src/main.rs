use std::fs;

use d6::solve::{solve_t1, solve_t2};

fn main() {
    let input = fs::read_to_string("./input").unwrap();
    print!("solution 1: {}", solve_t1(&input).unwrap());
    print!("solution 1: {}", solve_t2(&input));
    
}
