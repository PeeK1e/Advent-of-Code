use std::{
    fs,
    io::{BufRead, BufReader, Read},
    ptr::null,
    slice::Chunks,
};

fn lookup_table(chunk: &[u8]) -> i32 {
    return match chunk[..3] {
        [b'B', _, b'Z'] => 9,
        [b'A', _, b'Y'] => 8,
        [b'C', _, b'X'] => 7,
        [b'C', _, b'Z'] => 6,
        [b'B', _, b'Y'] => 5,
        [b'A', _, b'X'] => 4,
        [b'A', _, b'Z'] => 3,
        [b'C', _, b'Y'] => 2,
        [b'B', _, b'X'] => 1,
        _ => 0,
    };
}

fn lookup_table_p2(chunk: &[u8]) -> i32 {
    return match chunk[..3] {
        [b'A', _, b'Z'] => 8,
        [b'B', _, b'Z'] => 9,
        [b'C', _, b'Z'] => 7,
        [b'A', _, b'Y'] => 4,
        [b'B', _, b'Y'] => 5,
        [b'C', _, b'Y'] => 6,
        [b'A', _, b'X'] => 3,
        [b'B', _, b'X'] => 1,
        [b'C', _, b'X'] => 2,
        _ => 0,
    };
}

fn main() {
    let map = std::fs::read("./src/input.txt").unwrap();
    let result = map.chunks(4).fold(0, |acc, f| acc + lookup_table(f));
    let result_p2 = map.chunks(4).fold(0, |acc, f| acc + lookup_table_p2(f));
    println!("P1: {}, P2: {}", result, result_p2);
}
