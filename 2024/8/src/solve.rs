use std::collections::BTreeMap;

pub fn solve_t1(input: &str) -> Result<i32, String> {
    let map = make_map(input);

    let points: BTreeMap<(i32,i32), i32> = find_points(&map)?;

    let count = count_points(&points);

    Ok(count) 
}

fn make_map(input: &str) -> BTreeMap<String, Vec<(i32,i32)>> {
    let mut antennae: BTreeMap<String, Vec<(i32,i32)>> = BTreeMap::new();

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

    antennae
}

fn find_points(map: &BTreeMap<String, Vec<(i32,i32)>>) -> Result<BTreeMap<(i32,i32), i32>, String> {
    let mut points: BTreeMap<(i32,i32), i32> = BTreeMap::new();

    for antennae in map {
        let mut vector_points = vec![];
        vector_points.push(1);
    }

    return Err("No Points".to_string());
}

fn count_points(points: &BTreeMap<(i32,i32), i32>) -> i32 {
    let mut count = 0;
    for point in points {
        if !*point.1 > 1 {
            count += 1;
        }
    }
    return count;
}

pub fn solve_t2(input: &str) -> Result<i64, String> {
    let mut count = 0;

    Ok(count) 
}
