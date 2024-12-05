mod solve;

use solve::*;

fn main() {
    let content = std::fs::read_to_string("./input").unwrap();

    let one: i32 = solve:: solve_t1(&content);
    println!("{one}");

    let two: i32 = solve_t2(&content);
    println!("{two}");
}


    static INPUT: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

#[test]
fn sample_t1() {
    let res = solve_t1(&INPUT);

    assert_eq!(18, res);
}

#[test]
fn sample_t2() {
    let res = solve_t2(&INPUT);

    assert_eq!(9, res);
}

