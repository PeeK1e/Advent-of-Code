use std::{borrow::Borrow, cell::RefCell, collections::{hash_set, HashMap, HashSet}, rc::Rc};

struct Tile {
    id: RefCell<i32>,
    value: String,
    edges: RefCell<i8>,
    visited: RefCell<bool>,
    edge_id_up: RefCell<i32>,
    edge_id_right: RefCell<i32>,
    edge_id_down: RefCell<i32>,
    edge_id_left: RefCell<i32>,
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
        println!("Field {} has {} tiles with {} edges", value, field_count, edges);
    }

    Ok(count) 
}

#[allow(unused_variables,unused_mut)]
pub fn solve_t2(input: &str) -> Result<i64, String> {
    let mut count: i32 = 0;


    let map = make_map(input);

    find_equal_fields(&map);

    let mut corners: HashMap<i32, (usize, HashMap<i32,i32>, String)> = HashMap::new();

    for line in map {
        for field in line {
            let id = *field.id.borrow();
            let value = field.value.to_owned();
            let edge_id1 = *field.edge_id_down.borrow();
            let edge_id2 = *field.edge_id_up.borrow();
            let edge_id3 = *field.edge_id_left.borrow();
            let edge_id4 = *field.edge_id_right.borrow();

            if let Some(ele) = corners.get_mut(&id) {
                if edge_id1 != 0 {
                    ele.1.insert(edge_id1,1);
                }
                if edge_id2 != 0 {
                    ele.1.insert(edge_id2,1);
                }
                if edge_id3 != 0 {
                    ele.1.insert(edge_id3,1);
                }
                if edge_id4 != 0 {
                    ele.1.insert(edge_id4,1);
                }
                ele.0 += 1;
            } else {
                let mut ele = HashMap::new();
                if edge_id1 != 0 {
                    ele.insert(edge_id1,1);
                }
                if edge_id2 != 0 {
                    ele.insert(edge_id2,1);
                }
                if edge_id3 != 0 {
                    ele.insert(edge_id3,1);
                }
                if edge_id4 != 0 {
                    ele.insert(edge_id4,1);
                }
                corners.insert(id, (1,ele,value));
            }
        }
    }

    for (field_id, corner_id) in corners {
        let sum = corner_id.1.iter().map(|(_,i)| i).sum::<i32>();
        count +=  corner_id.0 as i32 * sum;
        println!("Field Letter {} / id: {} has {} edges with {} fields", corner_id.2, field_id, sum, corner_id.0);
    }

    Ok(count as i64) 
}

fn make_map(input: &str) -> Vec<Vec<Tile>> {
    let mut id = 0;
    let mut edge_id = 0;
    input
        .split("\n")
        .enumerate()
        .map(|(y, line)| {
            line
                .chars()
                .enumerate()
                .map(|(x,ch)|{
                    id += 1;
                    edge_id += 4;
                    Tile{
                        // this guarantees unique ID's
                        id: RefCell::new(id),
                        edges: RefCell::new(0),
                        value: ch.to_string(),
                        visited: RefCell::new(false),
                        edge_id_up: RefCell::new(edge_id),
                        edge_id_right: RefCell::new(edge_id+1),
                        edge_id_down: RefCell::new(edge_id+2),
                        edge_id_left: RefCell::new(edge_id+3),
                        //pos: (x as i32, y as i32),
                    }
                })
                .collect::<Vec<Tile>>()
        })
        .collect::<Vec<Vec<Tile>>>()
}

fn find_equal_fields(map: &Vec<Vec<Tile>>) {
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
                        tile.edge_id_left.replace(0);
                    }
                }
            }

            // check above
            if let Some(prev) = map.get((y as isize - 1) as usize) {
                if let Some(prev) = prev.get(x) {
                    if prev.value == tile.value {
                        edges -= 1;
                        tile.edge_id_up.replace(0);
                    }
                }
            }
            // check next
            if let Some(prev) = map.get(y) {
                if let Some(prev) = prev.get(x+1) {
                    if prev.value == tile.value {
                        edges -= 1;
                        tile.edge_id_right.replace(0);
                    }
                }
            }

            // check below
            if let Some(prev) = map.get(y+1) {
                if let Some(prev) = prev.get(x) {
                    if prev.value == tile.value {
                        edges -= 1;
                        tile.edge_id_down.replace(0);
                    }
                }
            }
            tile.edges.replace(edges);
        }
    } 
    for y in 0..map.len() {
        for x in 0..map.get(0).unwrap().len(){
            let tile = map.get(y).unwrap().get(x).unwrap();
            if tile.visited.borrow().clone() {
                continue;
            }
            tile.visited.replace(true);
            recursive_search(map, x, y, *tile.id.borrow(), &tile.value, *tile.edge_id_up.borrow(), *tile.edge_id_down.borrow(), *tile.edge_id_right.borrow(),*tile.edge_id_left.borrow());
        }
    }

    for _ in 0..10 {
        reset_visited(map);
        for y in (0..map.len()).rev() {
            for x in (0..map.get(0).unwrap().len()).rev(){
                let tile = map.get(y).unwrap().get(x).unwrap();
                if tile.visited.borrow().clone() {
                    continue;
                }
                tile.visited.replace(true);
                recursive_borders(map, x, y, *tile.id.borrow(), &tile.value, *tile.edge_id_up.borrow(), *tile.edge_id_down.borrow(), *tile.edge_id_right.borrow(),*tile.edge_id_left.borrow());
            }
        } 
    }
}

fn recursive_search(map: &Vec<Vec<Tile>>, map_x: usize, map_y: usize, id: i32, value: &str, up: i32, down: i32 , right: i32, left: i32) {
    // check next
    let (x,y) = (map_x+1,map_y);
    if let Some(prev) = map.get(y) {
        if let Some(prev) = prev.get(x) {
            if prev.value == value && !*prev.visited.borrow() {
                prev.id.replace(id);
                prev.visited.replace(true);
                recursive_search(map, x, y, id, value, *prev.edge_id_up.borrow(), *prev.edge_id_down.borrow(), *prev.edge_id_right.borrow(),*prev.edge_id_left.borrow());
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
                recursive_search(map, x, y, id, value, *prev.edge_id_up.borrow(), *prev.edge_id_down.borrow(), *prev.edge_id_right.borrow(),*prev.edge_id_left.borrow());
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
                recursive_search(map, x, y, id, value, *prev.edge_id_up.borrow(), *prev.edge_id_down.borrow(), *prev.edge_id_right.borrow(),*prev.edge_id_left.borrow());
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
                recursive_search(map, x, y, id, value, *prev.edge_id_up.borrow(), *prev.edge_id_down.borrow(), *prev.edge_id_right.borrow(),*prev.edge_id_left.borrow());
            }
        }
    }

}

fn reset_visited(map: &Vec<Vec<Tile>>) {
    for line in map {
        for tile in line {
            tile.visited.replace(false);
        }
    }
}

fn recursive_borders(map: &Vec<Vec<Tile>>, map_x: usize, map_y: usize, id: i32, value: &str, up: i32, down: i32 , right: i32, left: i32){ 
        // check previous
        let (x,y) = ((map_x as isize - 1) as usize, map_y);
        if let Some(prev) = map.get(y) {
            if let Some(prev) = prev.get(x) {
                if prev.value == value && !*prev.visited.borrow() {
                    if *prev.edge_id_down.borrow() != 0 {
                        if down != 0 {
                            prev.edge_id_down.replace(down);
                        }
                    }
                    if *prev.edge_id_up.borrow() != 0 {
                        if up != 0 {
                            prev.edge_id_up.replace(up);
                        }
                    }
                    if *prev.edge_id_left.borrow() != 0 {
                        if left != 0 {
                            prev.edge_id_left.replace(left);
                        }
                    }
                    if *prev.edge_id_right.borrow() != 0 {
                        if right != 0 {
                            prev.edge_id_right.replace(right);
                        }
                    }
                    prev.visited.replace(true);
                    recursive_borders(map, x, y, id, value, *prev.edge_id_up.borrow(), *prev.edge_id_down.borrow(), *prev.edge_id_right.borrow(),*prev.edge_id_left.borrow());
                }
            }
        }
    
        // check above
        let (x,y) = (map_x, (map_y as isize - 1) as usize);
        if let Some(prev) = map.get(y) {
            if let Some(prev) = prev.get(x) {
                if prev.value == value && !*prev.visited.borrow() {
                    if *prev.edge_id_down.borrow() != 0 {
                        if down != 0 {
                            prev.edge_id_down.replace(down);
                        }
                    }
                    if *prev.edge_id_up.borrow() != 0 {
                        if up != 0 {
                            prev.edge_id_up.replace(up);
                        }
                    }
                    if *prev.edge_id_left.borrow() != 0 {
                        if left != 0 {
                            prev.edge_id_left.replace(left);
                        }
                    }
                    if *prev.edge_id_right.borrow() != 0 {
                        if right != 0 {
                            prev.edge_id_right.replace(right);
                        }
                    }
                    prev.visited.replace(true);
                    recursive_borders(map, x, y, id, value, *prev.edge_id_up.borrow(), *prev.edge_id_down.borrow(), *prev.edge_id_right.borrow(),*prev.edge_id_left.borrow());
                }
            }
        }
    
        // check next
        let (x,y) = (map_x+1,map_y);
        if let Some(prev) = map.get(y) {
            if let Some(prev) = prev.get(x) {
                if prev.value == value && !*prev.visited.borrow() {
                    if *prev.edge_id_down.borrow() != 0 {
                        if down != 0 {
                            prev.edge_id_down.replace(down);
                        }
                    }
                    if *prev.edge_id_up.borrow() != 0 {
                        if up != 0 {
                            prev.edge_id_up.replace(up);
                        }
                    }
                    if *prev.edge_id_left.borrow() != 0 {
                        if left != 0 {
                            prev.edge_id_left.replace(left);
                        }
                    }
                    if *prev.edge_id_right.borrow() != 0 {
                        if right != 0 {
                            prev.edge_id_right.replace(right);
                        }
                    }
                    prev.visited.replace(true);
                    recursive_borders(map, x, y, id, value, *prev.edge_id_up.borrow(), *prev.edge_id_down.borrow(), *prev.edge_id_right.borrow(),*prev.edge_id_left.borrow());
                }
            }
        }
   

        // check below
        let (x,y) = (map_x, map_y+1);
        if let Some(prev) = map.get(y) {
            if let Some(prev) = prev.get(x) {
                if prev.value == value && !*prev.visited.borrow() {
                    if *prev.edge_id_down.borrow() != 0 {
                        if down != 0 {
                            prev.edge_id_down.replace(down);
                        }
                    }
                    if *prev.edge_id_up.borrow() != 0 {
                        if up != 0 {
                            prev.edge_id_up.replace(up);
                        }
                    }
                    if *prev.edge_id_left.borrow() != 0 {
                        if left != 0 {
                            prev.edge_id_left.replace(left);
                        }
                    }
                    if *prev.edge_id_right.borrow() != 0 {
                        if right != 0 {
                            prev.edge_id_right.replace(right);
                        }
                    }
                    prev.visited.replace(true);
                    recursive_borders(map, x, y, id, value, *prev.edge_id_up.borrow(), *prev.edge_id_down.borrow(), *prev.edge_id_right.borrow(),*prev.edge_id_left.borrow());
                }
            }
        }
    
}