use std::{str, usize};

pub fn solve_t1(input: &str) -> Result<i64, String> {
    let mut count = 0;
    let map = make_map(input);

    for (result, numbers) in map {
        let num = numbers[0];
        match permutate(result, &numbers, num, 0) {
            Some(x) => count += x,
            None => {},
        }
    }

    Ok(count) 
}

pub fn solve_t2(input: &str) -> Result<i64, String> {
    let mut count = 0;
    let map = make_map(input);

    for (result, numbers) in map {
        let num = numbers[0];
        match permutate2(result, &numbers, num, 0) {
            Some(x) => count += x,
            None => {},
        }
    }

    Ok(count) 
}

fn make_map(input: &str) -> Vec<(i64, Vec<i64>)> {
    input
        .split("\n")
        .map(|line| {
            let l: Vec<&str> = line
                .split(": ")
                .collect();
            let result = l[0].parse::<i64>().unwrap();
            let list = l[1].split(" ").map(|n| n.parse::<i64>().unwrap()).collect();
            (result, list)
        })
        .collect()
}

fn permutate(result: i64, numbers: &Vec<i64>, cache: i64, index: usize) -> Option<i64> {
    let mut index = index;
    if index+1 < numbers.len() {
        index+=1;
    } else {
        if cache == result {
            return Some(result);
        }
        return None;
    }

    let b = numbers.get(index).unwrap();

    let r1 = match permutate(result, &numbers, cache+b, index) {
        Some(x) => x,
        None => -1,
    };

    let r2 = match permutate(result, &numbers, cache*b, index) {
        Some(x) => x,
        None => -1,
    };
    
    if r1 != -1 {
        return Some(r1);
    }
    if r2 != -1 {
        return Some(r2);
    }
    return None;
}


fn permutate2(result: i64, numbers: &Vec<i64>, cache: i64, index: usize) -> Option<i64> {
    let mut index = index;
    if index+1 < numbers.len() {
        index+=1;
    } else {
        if cache == result {
            return Some(result);
        }
        return None;
    }

    let b = numbers.get(index).unwrap();

    let r1 = match permutate2(result, &numbers, cache+b, index) {
        Some(x) => x,
        None => -1,
    };

    let r2 = match permutate2(result, &numbers, cache*b, index) {
        Some(x) => x,
        None => -1,
    };

    let r3 = match permutate2(result, &numbers, connotate(cache,b.clone()), index) {
        Some(x) => x,
        None => -1,
    };
    
    if r1 != -1 {
        return Some(r1);
    }
    if r2 != -1 {
        return Some(r2);
    }
    if r3 != -1 {
        return Some(r3);
    }
    return None;
}

fn connotate(a: i64, b: i64) -> i64 {
    let a = a.to_string();
    let b = b.to_string();
    let c = a.to_string()+&b.to_string();

    c.parse::<i64>().unwrap()
}
