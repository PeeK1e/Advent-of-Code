use std::collections::BTreeMap;

pub fn solve_t1(input: &str) -> Result<i32, String> {
    let (map, x, y) = make_map(input);

    let points: BTreeMap<(i32,i32), i32> = find_points(&map, x,y)?;

    let count = points.len() as i32;

    print_map(input, points);

    Ok(count) 
}

pub fn solve_t2(input: &str) -> Result<i32, String> {
    let (map, x, y) = make_map(input);

    let mut points: BTreeMap<(i32,i32), i32> = find_points_2(&map, x,y)?;   

    for (_, antennae) in map {
        for antenna in antennae {
            points.insert(antenna, 1);
        }
    }
    
    let count = points.len() as i32;

    print_map(input, points);

    Ok(count)
}

fn make_map(input: &str) -> (BTreeMap<String, Vec<(i32,i32)>>, i32, i32) {
    let mut antennae: BTreeMap<String, Vec<(i32,i32)>> = BTreeMap::new();
    let split = input.split("\n").collect::<Vec<&str>>();

    let y = split.len() as i32;
    let x = split.get(0).unwrap().len() as i32;

    for (y, line) in input.split("\n").enumerate() {
        for (x , field) in line.chars().enumerate() {
            if field.to_string() != "." {
                match antennae.get_mut(&field.to_string()) {
                    Some(elem) => elem.push((x as i32, y as i32)),
                    None => {
                        let mut vec = vec![];
                        vec.push((x as i32, y as i32));
                        antennae.insert(field.to_string(), vec);
                    }
                }
            }
        }
    };

    (antennae, x, y)
}

fn find_points_2(map: &BTreeMap<String, Vec<(i32,i32)>>, limit_x: i32, limit_y: i32) -> Result<BTreeMap<(i32,i32), i32>, String> {
    let mut points: BTreeMap<(i32,i32), i32> = BTreeMap::new();

    for antennae in map {
        let mut vector_points: Vec<(i32,i32)> = vec![];
        let len = antennae.1.len();
        if len <= 1 {continue;}
        for i in 0..len-1 {
            for j in i+1..len {
                let point_a = antennae.1.get(i).unwrap();
                let point_b = antennae.1.get(j).unwrap();
                let vector = (point_a.0 - point_b.0, point_a.1 - point_b.1); 

                let mut mult = 1;
                loop {
                    let x = point_a.0 - (vector.0 * mult);
                    let y = point_a.1 - (vector.1 * mult);
                    if !in_bounds(x, y, limit_x, limit_y) {
                        break;
                    }
                    vector_points.push((x,y));
                    mult+=1;
                }
                let mut mult = 1;
                loop {
                    let x = point_a.0 + (vector.0 * mult);
                    let y = point_a.1 + (vector.1 * mult);
                    if !in_bounds(x, y, limit_x, limit_y) {
                        break;
                    }
                    vector_points.push((x,y));
                    mult+=1;
                }
                let mut mult = 1;
                loop {
                    let x = point_b.0 - (vector.0 * mult);
                    let y = point_b.1 - (vector.1 * mult);
                    if !in_bounds(x, y, limit_x, limit_y) {
                        break;
                    }
                    vector_points.push((x,y));
                    mult+=1;
                }
                let mut mult = 1;
                loop {
                    let x = point_b.0 + (vector.0 * mult);
                    let y = point_b.1 + (vector.1 * mult);
                    if !in_bounds(x, y, limit_x, limit_y) {
                        break;
                    }
                    vector_points.push((x,y));
                    mult+=1;
                }
            }
        }

        for point in vector_points {
            points.insert(point, 1);
        }
    }

    return Ok(points);
}

fn in_bounds(x: i32, y: i32, limit_x: i32, limit_y: i32) -> bool {
    if x >= limit_x || y >= limit_y || x < 0 || y < 0{
        return false;
    }
    return true;
}

fn find_points(map: &BTreeMap<String, Vec<(i32,i32)>>, limit_x: i32, limit_y: i32) -> Result<BTreeMap<(i32,i32), i32>, String> {
    let mut points: BTreeMap<(i32,i32), i32> = BTreeMap::new();

    for antennae in map {
        let mut vector_points: Vec<(i32,i32)> = vec![];
        let len = antennae.1.len();
        if len <= 1 {continue;}
        for i in 0..len-1 {
            for j in i+1..len {
                let point_a = antennae.1.get(i).unwrap();
                let point_b = antennae.1.get(j).unwrap();
                let vector = (point_a.0 - point_b.0, point_a.1 - point_b.1);

                let p1 = (point_a.0 - vector.0, point_a.1 - vector.1);
                let p2 = (point_a.0 + vector.0, point_a.1 + vector.1);
                let p3 = (point_b.0 - vector.0, point_b.1 - vector.1);
                let p4 = (point_b.0 + vector.0, point_b.1 + vector.1);

                if p1 == *point_b {
                    vector_points.push(p2);
                } else {
                    vector_points.push(p1);
                }

                if p3 == *point_b {
                    vector_points.push(p4);
                } else {
                    vector_points.push(p3);
                } 
            }
        }
        for point in vector_points {
            if !in_bounds(point.0, point.1, limit_x, limit_y) {
                continue;
            }
            points.insert(point, 1);
        }
    }

    return Ok(points);
}

fn print_map(input: &str, points: BTreeMap<(i32,i32), i32>) {
    for (y, line) in input.split("\n").enumerate() {
        println!();
        for (x, field) in line.chars().enumerate() {
            match points.get(&(x as i32,y as i32)) {
                Some(_) => {
                    if field.to_string() != "." {
                        print!("{field}")
                    } else {
                        print!("#")
                    }
                },
                None => print!("{field}"),
            }
        }
    }
    println!();
}
