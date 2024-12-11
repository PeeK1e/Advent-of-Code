use regex::Regex;

pub fn solve_t1(content: &str) -> i32 {
    let strings = make_strings(content);
    let rgx1 = Regex::new(r"(XMAS)").unwrap();
    let rgx2 = Regex::new(r"(SAMX)").unwrap();

    let mut count:i32 = 0;
    for s in &strings {
        rgx1.captures_iter(s).for_each(|_| count+=1 );
    }
    for s in &strings {
        rgx2.captures_iter(s).for_each(|_| count+=1 );
    }

    count
}

pub fn solve_t2(content: &str) -> i32 {
    let map = make_2d_vec(content);

    let mut count = 0;
    // only 1..n-1
    let row_len = map.get(0).unwrap().len();
    for y in 1..map.len()-1 {
        // only 1..n-1
        for x in 1..row_len-1 {
            if *map.get(y).unwrap().get(x).unwrap() == 'A' {
                count += search_mas(&map, x, y);
            }
        }
    }

    return count;
}

fn search_mas(map: &Vec<Vec<char>>, x: usize, y: usize) -> i32 {
    let tl = *map.get(y-1).unwrap().get(x-1).unwrap();
    let br = *map.get(y+1).unwrap().get(x+1).unwrap();

    let tr = *map.get(y-1).unwrap().get(x+1).unwrap();
    let bl = *map.get(y+1).unwrap().get(x-1).unwrap();

    let mut diagright = false; 
    let mut diagleft = false; 
    if tl == 'S' && br == 'M' || tl == 'M' && br == 'S' {
        diagright = true;
    }
    if bl == 'S' && tr == 'M' || bl == 'M' && tr == 'S' {
        diagleft = true;
    }

    if diagleft && diagright {
        return 1;
    }

    return 0;
}

fn make_strings(input: &str) -> Vec<String> {
    let mut strings = vec![];
    //horizontal
    input.lines().for_each(|l| strings.push(l.to_string()));
    
    let map = make_2d_vec(input);
    let len_row = map.get(0).unwrap().len();
    let len_cols = map.len();

    // vertically
    for x in 0..len_row as usize {
        let mut word = String::from("");
        for y in 0..len_cols {
            let character = map.get(y).unwrap().get(x).unwrap();
            word.push(character.clone());
        }
        strings.push(word);
    }

    // diagonally right
    for x in 0..len_row as usize {
        let mut word = String::from("");
        for y in 0..len_cols {
            if x + y > len_row-1 {
                break;
            }
            let character = map.get(y).unwrap().get(x+y).unwrap();
            word.push(character.clone());
        }
        strings.push(word);
    }

    // diagonally left
    for x in 0..len_row as usize {
        let mut word = String::from("");
        for y in 0..len_cols {
            let idx = len_row - 1 - x;
            let character = map.get(y).unwrap().get(idx-y).unwrap();
            word.push(character.clone());
            if idx-y == 0 {
                break;
            }
        }
        strings.push(word);
    }
    
    // diagonally right y = 1
    for y in 1..len_cols {
        let mut word = String::from("");
        for x in 0..len_row as usize {
            if x + y > len_cols-1 {
                break;
            }
            let character = map.get(x+y).unwrap().get(x).unwrap();
            word.push(character.clone());
        }
        strings.push(word);
    }

    // diagonally left y = 1
    for y in 1..len_cols {
        let mut word = String::from("");
        for x in 0..len_row as usize {
            if x + y > len_cols-1 {
                break;
            }
            let idx = len_row - 1 - x;  
            let character = map.get(x+y).unwrap().get(idx).unwrap();
            word.push(character.clone());
        }
        strings.push(word);
    }
    
    return strings;
}

fn make_2d_vec(input: &str) -> Vec<Vec<char>>{
    input
        .lines()
        .map(|l| {
            let mut items = vec![];
            for e in l.as_bytes() {
                items.push(*e as char);
            }
            items
    }).collect()
}