use std::collections::BTreeMap;

const TRAIL: [&str; 10] = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];
const TRAIL_BEGIN: &str = "0";
const TAIL_LEN: usize = 10;

#[allow(unused_variables,unused_mut,unused_assignments)]
pub fn solve_t1(input: &str) -> Result<i64, String> {
    let mut count = 0;

    let (map, h, w) = make_trail_map(input);

    let trails = find_trail(&map, h, w);

    let count = get_trail_head_scores(&trails);

    Ok(count) 
}

#[allow(unused_variables,unused_mut)]
pub fn solve_t2(input: &str) -> Result<i64, String> {
    let mut count = 0;

    Ok(count) 
}

fn make_trail_map(raw: &str) -> (Vec<Vec<String>>, usize, usize) {
    let map: Vec<Vec<String>> = raw
        .split("\n")
        .map(|line| {
            line
                .chars()
                .map(|c| c.to_string())
                .collect::<Vec<String>>()
        })
        .collect();

    let height = map.len();
    let width = map.get(0).unwrap().len();

    (map, height, width)
}

fn find_trail(map: &Vec<Vec<String>>, heigth: usize, width: usize) -> Vec<Vec<(usize,usize)>> {
    let mut trails: Vec<Vec<(usize,usize)>> = vec![vec![]];

    for y in 0..heigth {
        for x in 0..width {
            let c = map.get(y).unwrap().get(x).unwrap();
            match c.as_str() {
                TRAIL_BEGIN => {
                    if let Some(path) = find_path(map, x, y, heigth, width){
                        trails.push(path);
                    };
                }
                _ => continue,
            }
        }
    }
    trails
}

fn get_trail_head_scores(trails: &Vec<Vec<(usize,usize)>>) -> i64 {
    let mut tree: BTreeMap<(usize,usize), i64> = BTreeMap::new();
    for trail in trails {
        if let Some(head) = trail.last() {
            match tree.get_mut(head) {
                Some(ele) => {
                    *ele += 1;
                },
                None => {
                    tree.insert(head.clone(), 1);
                },
            }
        }
    }

    tree
        .iter()
        .map(|(_,i)| i )
        .sum()
}

fn find_path(map: &Vec<Vec<String>>, x: usize, y: usize, height: usize, width: usize) -> Option<Vec<(usize,usize)>>{
    let mut path = vec![];
    let mut check_x = x;
    let mut check_y = y;
    for i in 1..TAIL_LEN {
        match check_next_tile(map, i, check_x, check_y, height, width) {
            Some(x) => {
                path.push(x);
                (check_x, check_y) = x;
            },
            None => return None,
        }
    }
    Some(path)
}

fn check_next_tile(map: &Vec<Vec<String>>, step: usize, origin_x: usize, origin_y: usize, height: usize, width: usize) -> Option<(usize,usize)> {
    // up
    let (x,y) = (origin_x,(origin_y as isize - 1) as usize); 
    if is_not_oob(x, y, height, width) {
        let tile = map.get(y).unwrap().get(x).unwrap();
        if next_char_on_trail(tile, step) {
            return Some((x,y));
        }
    }
    // left
    let (x,y) = ((origin_x as isize - 1) as usize,origin_y);
    if is_not_oob(x, y, height, width) {
        let tile = map.get(y).unwrap().get(x).unwrap();
        if next_char_on_trail(tile, step) {
            return Some((x,y));
        }
    }

    // right
    let (x,y) = (origin_x+1,origin_y);
    if is_not_oob(x, y, height, width) {
        let tile = map.get(y).unwrap().get(x).unwrap();
        if next_char_on_trail(tile, step) {
            return Some((x,y));
        }
    } 
    // down
    let (x,y) = (origin_x,origin_y+1); 
    if is_not_oob(x, y, height, width) {
        let tile = map.get(y).unwrap().get(x).unwrap();
        if next_char_on_trail(tile, step) {
            return Some((x,y));
        }
    }

    None
}

fn next_char_on_trail(map_tile: &str, step: usize) -> bool {
    let next_ok_tile = TRAIL[step];
    if map_tile == next_ok_tile {
        return true;
    }
    return false;
}

fn is_not_oob(x: usize, y: usize, height: usize, width: usize) -> bool {
    if x >= isize::MAX as usize || y >= isize::MAX as usize {
        return false;
    }
    if x >= width || y >= height {
        return false;
    }
    return true;
}