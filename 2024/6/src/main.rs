use std::fs;

use d6::solve::{solve_t1, solve_t2};

static INPUT: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";


fn main() {
    let input = fs::read_to_string("./input").unwrap();
    println!("solution 1: {}", solve_t1(&input).unwrap());
    let input = fs::read_to_string("./input.txt").unwrap();
    println!("solution 2: {}", solve_t2(&input).unwrap());
}
