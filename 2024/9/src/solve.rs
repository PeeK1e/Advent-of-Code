use std::vec;

pub fn solve_t1(input: &str) -> Result<i64, String> {
    let mut decompressed = decompress(input);

    let defrag = defragment(&mut decompressed);
    let count = checksum(&defrag);

    Ok(count) 
}

pub fn solve_t2(input: &str) -> Result<i64, String> {
    let mut decompressed = decompress(input);

    let defrag = move_blocks(&mut decompressed);
    let count = checksum_block(&defrag);

    Ok(count) 
}

fn decompress(input: &str) -> Vec<Option<i32>> {
    let mut file_id = 0;
    let mut values = vec![];
    input
        .chars()
        .enumerate()
        .for_each(|(i,f)| {
            let digit = f.to_digit(10).unwrap() as i32;
            if i%2 == 0 {
                for _ in 0..digit {
                    values.push(Some(file_id));
                }
                file_id += 1;
            } else {
                for _ in 0..digit {
                    values.push(None);
                }
            }
        });
    values
}

fn defragment(orig: &Vec<Option<i32>>) -> Vec<i32> {
    let mut data= orig.clone();
    let mut defraged = vec![];
    let mut idx = data.len(); 
    for i in 0..data.len() {
        let ele = data.get(i).unwrap();
        match *ele {
            None => {
                idx = find_last(&data, idx);
                if idx <= i { break };

                data.swap(idx, i);
            },
            Some(_) => {
                continue;
            },
        }
    }

    for e in data.iter() {
        match e {
            Some(v) => defraged.push(v.clone()),
            None => continue, 
        }
    }
    defraged

}

fn move_blocks(orig: &Vec<Option<i32>>) -> Vec<Option<i32>> {
    let mut data = orig.clone();

    let mut idx = data.len();
    while idx != 0 {
        //print_data_option(&data);
        let (b_start, b_size) = find_block(&data, idx);
        idx = b_start;

        match find_free_block_with_size(&data, b_size) {
            None => continue,
            Some((begin, _)) => {
                if begin > b_start {
                    continue;
                }

                for i in 0..b_size {
                    data.swap(begin+i, b_start+i);
                }
            }
        }
    }

    data
}

fn checksum(data: &Vec<i32>) -> i64 {
    let mut sum = 0;
    for (i, ele) in data.iter().enumerate() {
        sum += (*ele as i64) * i as i64;
    }
    return sum;
}

fn checksum_block(data: &Vec<Option<i32>>) -> i64 {
    let mut sum = 0;
    for (i, opt) in data.iter().enumerate() {
        match opt {
            Some(ele) => sum += (*ele as i64) * i as i64,
            None => continue,
        }
    }
    return sum;
}

fn find_last(data: &Vec<Option<i32>>, start_at: usize) -> usize {
    for i in (0..start_at).rev() {
        match data.get(i).unwrap() {
            None => continue,
            Some(_) => return i,
        }
    }

    return 0;
}

fn find_block(data: &Vec<Option<i32>>, start_at: usize) -> (usize, usize) {
    let mut symbol = &0;
    let mut end_block = data.len();
    for i in (0..start_at).rev() {
        match data.get(i).unwrap() {
            None => continue,
            Some(e) => {symbol = e; end_block = i; break;},
        }
    }
    let mut size = 1;
    let mut idx = end_block;
    while idx != 0 {
        idx-=1;
        match data.get(idx).unwrap() {
            Some(e) => {
                if e == symbol {
                    size += 1;
                } else {
                    break;
                }
            },
            None => break,
        }
    }
    let start = end_block+1 - size;

    return (start, size);
}

fn find_free_block_with_size(data: &Vec<Option<i32>>, size: usize) -> Option<(usize, usize)> {
    let mut index = 0;
    let mut search_start= 0;

    loop {
        let mut s_size = 0;
        if index >= data.len() {
            return None;
        }
        match data.get(index).unwrap() {
            Some(_) => {},
            None => {
                let mut idx = index;
                search_start = index;
                loop {
                    idx+=1;
                    if idx >= data.len() {
                        break;
                    }
                    s_size+=1;
                    
                    match data.get(idx).unwrap() {
                        Some(_) => break,
                        None => continue,
                    }
                }
            }
        }

        index += 1;

        if s_size >= size {
            return Some((search_start, s_size));
        }
    }
}

// fn print_data_option(data: &Vec<Option<i32>>) {
//     for e in data {
//         match e {
//            Some(v) => print!("{v}"),
//            None => print!("."), 
//         }
//     }
//     println!()
// }