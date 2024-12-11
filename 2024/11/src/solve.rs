use std::{collections::BTreeMap, sync::{mpsc, Arc}, thread};


#[allow(unused_variables,unused_mut)]
pub fn solve_t1(input: &str) -> Result<u64, String> {
    let mut count = 0;

    let stones = make_stones(input);

    count = blink(&stones, 25);

    Ok(count)
}

#[allow(unused_variables,unused_mut)]
pub fn solve_t2(input: &str) -> Result<u64, String> {
    let mut count = 0;

    let stones = make_stones(input);

    count = blink(&stones, 75);

    Ok(count)
}

fn make_stones(input: &str) -> BTreeMap<u64, u64> {
    input
        .split(" ")
        .map(|stone| {
            (stone.parse::<u64>().unwrap(), 1)
        })
        .collect::<BTreeMap<u64,u64>>()
}

fn blink(stones: &BTreeMap<u64, u64>, n: u64) -> u64 {
    let mut stones = stones.clone();

    for _ in 0..n {
        let mut blinked: BTreeMap<u64,u64> = BTreeMap::new();
        for (stone, count) in &stones {
            let (l,r) = blink_stone(*stone);

            if let Some(ele) = blinked.get_mut(&l) {
                *ele += count;
            } else {
                blinked.insert(l, *count);
            }

            if let Some(r) = r {
                if let Some(ele) = blinked.get_mut(&r) {
                    *ele += count;
                } else {
                    blinked.insert(r, *count);
                }
            }
        }
        stones = blinked;
    }
    
    stones
        .iter()
        .map(|(_,v)| v)
        .sum()
}

fn blink_stone(n: u64) -> (u64, Option<u64>) {
    if n == 0 {
        return (1,None);
    }
    if n >= 10 && letter_count(n) % 2 == 0 {
        let (l, r) = split_number(n);
        return (l, Some(r));
    }
    return (n*2024, None);
}

fn letter_count(num: u64) -> u64 {
    let mut num = num;
    if num == 0 {
        return 1;
    }
    let mut count = 0;
    while num > 0 {
        count += 1;
        num /= 10;
    }
    count
}

fn split_number(num: u64) -> (u64, u64) {
    let length = letter_count(num);

    let divisor = 10u64.pow((length/2) as u32); // 10^half_length

    let left = num / divisor;   // Linke Hälfte
    let right = num % divisor; // Rechte Hälfte

    (left, right)
}

fn print_stones(stones: &BTreeMap<u64,u64>) {
    println!("---");
    stones
        .iter()
        .for_each(|(i,v)| println!("{}|{}", i , v));

    println!()
}