use std::fs::{File};
use std::io::{BufRead, BufReader};
use std::ops::AddAssign;


fn main() {
    let mut count = 0;
    let mut elves = vec![0];
    let file = File::open("/home/ryuko/git-private/advent-of-code-2022/day-1/src/input").unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let l = line.unwrap();
        if l != "" {
            elves[count].add_assign(l.parse::<i32>().unwrap())
        }else{
            count.add_assign(1);
            elves.push(0);
        }
    }

    let max =  elves.iter().max();

    match max {
        Some(max) => println!("{}", max),
        None => println!("empty"),
    }
}
