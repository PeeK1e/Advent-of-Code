use std::{cell::RefCell, collections::HashMap, rc::Rc};

struct Tile {
    id: RefCell<i32>,
    value: String,
    edges: RefCell<i8>,
    visited: RefCell<bool>,
    //pos: (i32,i32),
}

#[allow(unused_variables,unused_mut)]
pub fn solve_t1(input: &str) -> Result<i64, String> {
    let mut count = 0;

    let map = make_map(input);

    find_equal_fields(&map);

    let mut hashmap: HashMap<i32, (usize,usize,String)> = HashMap::new();

    for line in map {
        for field in line {
            let value = field.value.to_owned();
            let edges = *field.edges.borrow() as usize;
            let id = *field.id.borrow();

            if let Some(ele) = hashmap.get_mut(&id) {
                ele.0 += 1;
                ele.1 += edges as usize;
            } else {
                hashmap.insert(id, (1, edges, value));
            }
        }
    }

    for (_, (field_count, edges, value)) in hashmap {
        count +=  (field_count * edges) as i64;
        //println!("Field {} has {} tiles with {} edges", value, field_count, edges);
    }

    Ok(count) 
}

#[allow(unused_variables,unused_mut)]
pub fn solve_t2(input: &str) -> Result<i64, String> {
    let mut count = 0;

    Ok(count) 
}

fn make_map(input: &str) -> Vec<Vec<Tile>> {
    let mut id = 0; 
    input
        .split("\n")
        .enumerate()
        .map(|(y, line)| {
            line
                .chars()
                .enumerate()
                .map(|(x,ch)|{
                    id += 1;
                    Tile{
                        // this guarantees unique ID's
                        id: RefCell::new(id),
                        edges: RefCell::new(0),
                        value: ch.to_string(),
                        visited: RefCell::new(false),
                        //pos: (x as i32, y as i32),
                    }
                })
                .collect::<Vec<Tile>>()
        })
        .collect::<Vec<Vec<Tile>>>()
}

fn find_equal_fields(map: &Vec<Vec<Tile>>) {
    for y in 0..map.len() {
        for x in 0..map.get(0).unwrap().len(){
            let tile = map.get(y).unwrap().get(x).unwrap();
            if tile.visited.borrow().clone() {
                continue;
            }
            tile.visited.replace(true); 
            recursive_search(map, x, y, *tile.id.borrow(), &tile.value);
        }
    }

    // setting edge count
    for y in 0..map.len() {
        for x in 0..map.get(0).unwrap().len(){
            let tile = map.get(y).unwrap().get(x).unwrap();
            let mut edges = 4;

            // check previous
            if let Some(prev) = map.get(y) {
                if let Some(prev) = prev.get((x as isize - 1) as usize) {
                    if prev.value == tile.value {
                        edges -= 1;
                    }
                }
            }

            // check above
            if let Some(prev) = map.get((y as isize - 1) as usize) {
                if let Some(prev) = prev.get(x) {
                    if prev.value == tile.value {
                        edges -= 1;
                    }
                }
            }
            // check next
            if let Some(prev) = map.get(y) {
                if let Some(prev) = prev.get(x+1) {
                    if prev.value == tile.value {
                        edges -= 1;
                    }
                }
            }

            // check below
            if let Some(prev) = map.get(y+1) {
                if let Some(prev) = prev.get(x) {
                    if prev.value == tile.value {
                        edges -= 1;
                    }
                }
            }

            tile.edges.replace(edges);
        }
    }
}

fn recursive_search(map: &Vec<Vec<Tile>>, map_x: usize, map_y: usize, id: i32, value: &str) {
    // check next
    let (x,y) = (map_x+1,map_y);
    if let Some(prev) = map.get(y) {
        if let Some(prev) = prev.get(x) {
            if prev.value == value && !*prev.visited.borrow() {
                prev.id.replace(id);
                prev.visited.replace(true);
                recursive_search(map, x, y, id, value);
            }
        }
    }

    // check previous
    let (x,y) = ((map_x as isize - 1) as usize, map_y);
    if let Some(prev) = map.get(y) {
        if let Some(prev) = prev.get(x) {
            if prev.value == value && !*prev.visited.borrow() {
                prev.id.replace(id);
                prev.visited.replace(true);
                recursive_search(map, x, y, id, value);
            }
        }
    }


    // check above
    let (x,y) = (map_x, (map_y as isize - 1) as usize);
    if let Some(prev) = map.get(y) {
        if let Some(prev) = prev.get(x) {
            if prev.value == value && !*prev.visited.borrow() {
                prev.id.replace(id);
                prev.visited.replace(true);
                recursive_search(map, x, y, id, value);
            }
        }
    }

    // check below
    let (x,y) = (map_x, map_y+1);
    if let Some(prev) = map.get(y) {
        if let Some(prev) = prev.get(x) {
            if prev.value == value && !*prev.visited.borrow() {
                prev.id.replace(id);
                prev.visited.replace(true);
                recursive_search(map, x, y, id, value);
            }
        }
    }

}