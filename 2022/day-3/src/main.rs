fn main() {
    let file = std::fs::read("./src/input.txt").unwrap();
    let input: Vec<&[u8]> = file
        .split(|char| char == &b'\n')
        .map(|e| e as &[u8])
        .collect();

    let mut sum: i32 = 0;

    let mut sum_2: i32 = 0;
    let mut p2_found1: Vec<i32> = vec![];
    let mut p2_found2: Vec<i32> = vec![];

    for (i, line) in input.iter().enumerate() {
        let (l, r) = line.split_at(line.len() / 2);
        sum += get_item_prio(l, r).get(0).unwrap();

        match i % 3 {
            0 => {
                p2_found1 = get_item_prio(line, input.get(i + 1).unwrap());
            }
            1 => {
                p2_found2 = get_item_prio(line, input.get(i + 1).unwrap());
            }
            2 => {
                let l1: Vec<u8> = p2_found1.iter().map(|e| *e as u8).collect();
                let l2: Vec<u8> = p2_found2.iter().map(|e| *e as u8).collect();
                sum_2 += get_item_prio_but_no_ascii_bullsh(l1.as_slice(), l2.as_slice());
            }
            _ => {}
        }
    }

    println!("SUM {} :: SUM2 {}", sum, sum_2);
}

fn get_item_prio(l: &[u8], r: &[u8]) -> Vec<i32> {
    let mut found: Vec<i32> = vec![];
    for e in l {
        if r.contains(e) {
            if e.is_ascii_lowercase() {
                if !found.contains(&((*e as i32) - 96)) {
                    found.push((*e as i32) - 96);
                }
            } else {
                if !found.contains(&((*e as i32) - 65 + 27)) {
                    found.push((*e as i32) - 65 + 27);
                }
            }
        }
    }
    return found;
}

fn get_item_prio_but_no_ascii_bullsh(l: &[u8], r: &[u8]) -> i32 {
    for e in l {
        if r.contains(e) {
            return *e as i32;
        }
    }
    return 0;
}
