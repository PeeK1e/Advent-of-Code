use std::fs;

use solution::solve_1::solve1;
use solution::solve_2::solve2;

fn main() {
    let input = fs::read_to_string("./input").unwrap();
    println!("solution 1: {}", solve1(&input).unwrap());
    println!("solution 2: {}", solve2(&input).unwrap());
}
