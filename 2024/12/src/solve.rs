use std::{borrow::Borrow, cell::RefCell, collections::{hash_set, HashMap, HashSet}, rc::Rc};

struct Tile {
    id: RefCell<i32>,
    value: String,
    edges: RefCell<i8>,
    visited: RefCell<bool>,
    edge_id_up: RefCell<(i32,bool)>,
    edge_id_right: RefCell<(i32,bool)>,
    edge_id_down: RefCell<(i32,bool)>,
    edge_id_left: RefCell<(i32,bool)>,
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
                if edge_id1.0 != 0 {
                    ele.1.insert(edge_id1.0,1);
                }
                if edge_id2.0 != 0 {
                    ele.1.insert(edge_id2.0,1);
                }
                if edge_id3.0 != 0 {
                    ele.1.insert(edge_id3.0,1);
                }
                if edge_id4.0 != 0 {
                    ele.1.insert(edge_id4.0,1);
                }
                ele.0 += 1;
            } else {
                let mut ele = HashMap::new();
                if edge_id1.0 != 0 {
                    ele.insert(edge_id1.0,1);
                }
                if edge_id2.0 != 0 {
                    ele.insert(edge_id2.0,1);
                }
                if edge_id3.0 != 0 {
                    ele.insert(edge_id3.0,1);
                }
                if edge_id4.0 != 0 {
                    ele.insert(edge_id4.0,1);
                }
                corners.insert(id, (1,ele,value));
            }
        }
    }

    for (field_id, corner_id) in corners {
        let sum = corner_id.1.iter().map(|(_,i)| i).sum::<i32>();
        count +=  corner_id.0 as i32 * sum;
        //println!("Field Letter {} / id: {} has {} edges with {} fields", corner_id.2, field_id, sum, corner_id.0);
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
                        edge_id_up: RefCell::new((edge_id,false)),
                        edge_id_right: RefCell::new((edge_id+1,false)),
                        edge_id_down: RefCell::new((edge_id+2,false)),
                        edge_id_left: RefCell::new((edge_id+3,false)),
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

            for (i, (dx,dy)) in [(1,0),(-1,0),(0,1),(0,-1)].iter().enumerate() {
                let (x,y) = ((x as isize + dx) as usize, (y as isize + dy) as usize);
                if let Some(prev) = map.get(y) {
                    if let Some(prev) = prev.get(x) {
                        if prev.value == tile.value {
                            edges -= 1;
                            match i {
                                0 =>  tile.edge_id_right.replace((0,true)),
                                1 =>  tile.edge_id_left.replace((0,true)),
                                2 =>  tile.edge_id_down.replace((0,true)),
                                3 =>  tile.edge_id_up.replace((0,true)),
                                //this should never trigger
                                _ => tile.edge_id_right.replace((0,true)),
                            };
                        }
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
            recursive_search(map, x, y, *tile.id.borrow(), &tile.value);
        }
    }

    for y in 0..map.len() {
        for x in 0..map.get(0).unwrap().len(){
            let tile = map.get(y).unwrap().get(x).unwrap();
            if tile.edge_id_down.borrow().1 {
                continue;
            }
            if tile.edge_id_down.borrow().0 != 0 {
                let id = tile.edge_id_down.borrow().0;
                tile.edge_id_down.replace((id,true));
                recursive_borders_down(map, x, y, id, &tile.value);
            }
        }
    }

    for y in 0..map.len() {
        for x in 0..map.get(0).unwrap().len(){
            let tile = map.get(y).unwrap().get(x).unwrap();
            if tile.edge_id_up.borrow().1 {
                continue;
            }
            if tile.edge_id_up.borrow().0 != 0 {
                let id = tile.edge_id_up.borrow().0;
                tile.edge_id_up.replace((id,true));
                recursive_borders_up(map, x, y, id, &tile.value);
            }
        }
    }

    for y in 0..map.len() {
        for x in 0..map.get(0).unwrap().len(){
            let tile = map.get(y).unwrap().get(x).unwrap();
            if tile.edge_id_left.borrow().1 {
                continue;
            }
            if tile.edge_id_left.borrow().0 != 0 {
                let id = tile.edge_id_left.borrow().0;
                tile.edge_id_left.replace((id,true));
                recursive_borders_left(map, x, y, id, &tile.value);
            }
        }
    }
    for y in 0..map.len() {
        for x in 0..map.get(0).unwrap().len(){
            let tile = map.get(y).unwrap().get(x).unwrap();
            if tile.edge_id_right.borrow().1 {
                continue;
            }
            if tile.edge_id_right.borrow().0 != 0 {
                let id = tile.edge_id_right.borrow().0;
                tile.edge_id_right.replace((id,true));
                recursive_borders_right(map, x, y, id, &tile.value);
            }
        }
    }
}

fn recursive_search(map: &Vec<Vec<Tile>>, map_x: usize, map_y: usize, id: i32, value: &str) {
    for (x,y) in [(1,0),(-1,0),(0,1),(0,-1)].iter() {
        let (x,y) = ((map_x as isize+x) as usize, (map_y as isize+y) as usize);
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
}

// fn reset_visited(map: &Vec<Vec<Tile>>) {
//     for line in map {
//         for tile in line {
//             tile.visited.replace(false);
//         }
//     }
// }

fn recursive_borders_down(map: &Vec<Vec<Tile>>, map_x: usize, map_y: usize, id: i32, value: &str){
    for (x,y) in [(0,-1),(1,0),(-1,0),(0,1)].iter() {
        let (x,y) = ((map_x as isize+x) as usize, (map_y as isize+y) as usize); 
        if let Some(prev) = map.get(y) {
            if let Some(prev) = prev.get(x) {
                if prev.value == value && !prev.edge_id_down.borrow().1 {
                    if prev.edge_id_down.borrow().0 != 0 {
                        prev.edge_id_down.replace((id,true));
                        recursive_borders_down(map, x, y, id, value);
                    }
                }
            }
        }
    }
}

fn recursive_borders_up(map: &Vec<Vec<Tile>>, map_x: usize, map_y: usize, id: i32, value: &str){
    for (x,y) in [(0,-1),(1,0),(-1,0),(0,1)].iter() {
        let (x,y) = ((map_x as isize+x) as usize, (map_y as isize+y) as usize); 
        if let Some(prev) = map.get(y) {
            if let Some(prev) = prev.get(x) {
                if prev.value == value && !prev.edge_id_up.borrow().1 {
                    if prev.edge_id_up.borrow().0 != 0 {
                        prev.edge_id_up.replace((id,true));
                        recursive_borders_up(map, x, y, id, value);
                    }
                }
            }
        }
    }
}
fn recursive_borders_left(map: &Vec<Vec<Tile>>, map_x: usize, map_y: usize, id: i32, value: &str){
    for (x,y) in [(0,-1),(1,0),(-1,0),(0,1)].iter() {
        let (x,y) = ((map_x as isize+x) as usize, (map_y as isize+y) as usize); 
        if let Some(prev) = map.get(y) {
            if let Some(prev) = prev.get(x) {
                if prev.value == value && !prev.edge_id_left.borrow().1 {
                    if prev.edge_id_left.borrow().0 != 0 {
                        prev.edge_id_left.replace((id,true));
                        recursive_borders_left(map, x, y, id, value);
                    }
                }
            }
        }
    }
}
fn recursive_borders_right(map: &Vec<Vec<Tile>>, map_x: usize, map_y: usize, id: i32, value: &str){
    for (x,y) in [(0,-1),(1,0),(-1,0),(0,1)].iter() {
        let (x,y) = ((map_x as isize+x) as usize, (map_y as isize+y) as usize); 
        if let Some(prev) = map.get(y) {
            if let Some(prev) = prev.get(x) {
                if prev.value == value && !prev.edge_id_right.borrow().1 {
                    if prev.edge_id_right.borrow().0 != 0 {
                        prev.edge_id_right.replace((id,true));
                        recursive_borders_right(map, x, y, id, value);
                    }
                }
            }
        }
    }
}