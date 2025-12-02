use core::str;


#[allow(unused_variables,unused_mut)]
pub fn solve_t1(input: &str) -> Result<i64, String> {
    let mut count = 0;

    let mut invalid = vec![];
    for ele in split_input(input) {

        for i in ele.0..=ele.1 {
            let s = i.to_string();
            let split = s.split_at(s.len()/2);
    
            if split.0 == split.1 {
                invalid.push(i);
            }
        }
    }

    for ele in invalid {
        count += ele;
    }

    Ok(count) 
}

#[allow(unused_variables,unused_mut)]
pub fn solve_t2(input: &str) -> Result<i64, String> {
    let mut count = 0;

    Ok(count) 
}

fn split_input(inp: &str) -> Vec<(i64, i64)> {
    let mut pair_list: Vec<(i64, i64)> = vec![];
    for ele in inp.split(",") {
        let mut row = ele.split("-");
        let ele1 = row.next().unwrap().parse::<i64>().unwrap();
        let ele2 = row.next().unwrap().parse::<i64>().unwrap();
        pair_list.push((ele1, ele2));
    }
    return pair_list;
}