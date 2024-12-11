use std::{sync::{mpsc, Arc}, thread};


#[allow(unused_variables,unused_mut)]
pub fn solve_t1(input: &str) -> Result<u64, String> {
    let mut count = 0;

    let stones = make_stones(input);

    let (tx,rx) = mpsc::channel();
    let txarc = Arc::new(tx);

    let len = stones.len();
    for stone in stones {   
        let copy_tx = txarc.clone();
        thread::spawn(move || {
            println!("Thread Spawned");
            let val = blink2([stone, 0], 1, 0, 25);
            copy_tx.send(val).unwrap();
        });
    }

    for _ in 0..len {
        count += rx.recv().unwrap();
        println!("got message");
    }

    Ok(count)
}

#[allow(unused_variables,unused_mut)]
pub fn solve_t2(input: &str) -> Result<u64, String> {
    let mut count = 0;

    let stones = make_stones(input);

    let (tx,rx) = mpsc::channel();
    let txarc = Arc::new(tx);

    let len = stones.len();
    for stone in stones {   
        let copy_tx = txarc.clone();
        thread::spawn(move || {
            println!("Thread Spawned");
            let val = blink2([stone, 0], 1, 0, 75);
            copy_tx.send(val).unwrap();
        });
    }

    for _ in 0..len {
        count += rx.recv().unwrap();
        println!("got message");
    }

    Ok(count)
}

fn make_stones(input: &str) -> Vec<u64> {
    input
        .split(" ")
        .map(|stone| {
            stone.parse::<u64>().unwrap()
        })
        .collect::<Vec<u64>>()
}

fn blink2(stones: [u64; 2], len: usize, n: u64, stop: u64) -> u64 {
    if n >= stop {
        return len as u64;
    }

    let mut count = 0;
    for i in 0..len {
        if stones[i] == 0 {
            count += blink2([1,0], 1, n+1, stop);
        } else if letter_count(stones[i]) % 2 == 0 && stones[i] > 9 {
            let (l,r) = split_number(stones[i]);
            count += blink2([l,r], 2, n+1, stop);
        } else {
            count += blink2( [(stones[i]*2024) as u64, 0], 1, n+1, stop);
        }
    }
    return count;
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
