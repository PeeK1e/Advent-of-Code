use std::{i64};


#[allow(unused_variables,unused_mut)]
pub fn solve_t1(input: &str) -> Result<i64, String> {
    let mut count: i64 = 0;

    let banks = read_and_sort(input);

    for ele in banks {
        let idx1= find_largest_idx(&ele, 0);

        if idx1 == ele.len()-1 {
            let mut tmp = ele.clone();
            tmp.remove(idx1);
            let idx2 = find_largest_idx(&tmp, 0);
            count += concat(tmp.get(idx2).unwrap(), ele.get(idx1).unwrap());
        } else {
            let idx2 = find_largest_idx(&ele, idx1+1);
            count += concat(ele.get(idx1).unwrap(), ele.get(idx2).unwrap());
        }
    }

    Ok(count) 
}

#[allow(unused_variables,unused_mut)]
pub fn solve_t2(input: &str) -> Result<i64, String> {
    let mut count = 0;

    Ok(count) 
}

fn read_and_sort(inp: &str) -> Vec<Vec<i64>> {
    let mut banks = vec![];

    for ele in inp.split("\n") {
        let line: Vec<i64> = ele.chars().map(|x| x.to_digit(10).unwrap() as i64 ).collect();
        banks.push(line);
    }
    return banks;
}

fn find_largest_idx(ele: &Vec<i64>, start_idx: usize) -> usize {
    if start_idx+1 >= ele.len() {
        return 0;
    }

    let mut idx = start_idx;
    for i in start_idx+1..ele.len() {
        let l = ele.get(idx).unwrap();
        let r = ele.get(i).unwrap();

        if l.lt(r) {
            idx = i;
        }
    }
    idx
}

fn concat(a: &i64, b: &i64) -> i64 {
    let s = a.to_string() + b.to_string().as_str();
    return s.parse::<i64>().unwrap();
}