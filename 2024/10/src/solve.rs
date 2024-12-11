use std::{collections::BTreeMap, f32::consts::E, path};

struct Tile {
    value: u8,
    visited: bool,
}

#[allow(unused_variables,unused_mut,unused_assignments)]
pub fn solve_t1(input: &str) -> Result<i64, String> {
    let mut count = 0;

    let chart = make_chart(input);

    let mut trail_starts = vec![];

    for (y,line) in chart.iter().enumerate() {
        for (x, field) in line.iter().enumerate() {
            if *field == 0 {
                trail_starts.push((x,y));
            }
        }
    } 
    
    for (x,y) in trail_starts {
        let mut points: BTreeMap<(usize,usize), usize> = BTreeMap::new();
        search_path_endpoint(&chart, x, y, &mut points);
        let c = points.len();
        println!("{c}");
        count += c;
    } 

    Ok(count as i64) 
}

#[allow(unused_variables,unused_mut)]
pub fn solve_t2(input: &str) -> Result<i64, String> {
    let mut count = 0;

    let chart = make_chart(input);

    let mut trail_starts = vec![];

    for (y,line) in chart.iter().enumerate() {
        for (x, field) in line.iter().enumerate() {
            if *field == 0 {
                trail_starts.push((x,y));
            }
        }
    } 
    
    for (x,y) in trail_starts {
        let mut points: BTreeMap<(usize,usize), usize> = BTreeMap::new();
        search_path_endpoint(&chart, x, y, &mut points);
        let c = points.iter().map(|(_,i)| *i as i64).sum::<i64>();
        count += c;
    } 


    Ok(count) 
}

fn make_chart(input: &str) -> Vec<Vec<u8>> {
    input
        .split("\n")
        .map(|line| {
            line
                .chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<Vec<u8>>>()
}

fn search_path_endpoint(chart: &Vec<Vec<u8>>, x: usize, y: usize, points: &mut BTreeMap<(usize,usize), usize>) {
    let current_tile = chart.get(y).unwrap().get(x).unwrap();

    if *current_tile == 9 {
        if let Some(n) = points.get_mut(&(x,y)) {
            *n += 1;
        } else {
            points.insert((x,y), 1);
        }
        return;
    }

    //up
    if let Some((x,y)) = is_ok(chart, x as isize, y as isize - 1) {
        let tile = *chart.get(y).unwrap().get(x).unwrap();
        if  tile as isize - *current_tile as isize == 1 && *current_tile < tile {
            search_path_endpoint(chart, x, y, &mut *points);
        }
    }
    //right
    if let Some((x,y)) = is_ok(chart, (x+1) as isize, y as isize) {
        let tile = *chart.get(y).unwrap().get(x).unwrap();
        if tile as isize - *current_tile as isize == 1 && *current_tile < tile {
            search_path_endpoint(chart, x, y, &mut *points);
        }
    }
    //down
    if let Some((x,y)) = is_ok(chart, x as isize, (y+1) as isize) {
        let tile = *chart.get(y).unwrap().get(x).unwrap();
        if  tile as isize - *current_tile as isize == 1 && *current_tile < tile {
            search_path_endpoint(chart, x, y, &mut *points);
        }
    }
    //left
    if let Some((x,y)) = is_ok(chart, x as isize - 1, y as isize) {
        let tile = *chart.get(y).unwrap().get(x).unwrap();
        if  tile as isize - *current_tile as isize == 1 && *current_tile < tile {
            search_path_endpoint(chart, x, y, &mut *points);
        }
    }
}

fn is_ok(chart: &Vec<Vec<u8>>, x: isize, y: isize) -> Option<(usize,usize)> {
    let bounds_x = chart.get(0).unwrap().len();
    let bounds_y = chart.len();

    if x < 0 || y < 0 {
        return None;
    }
    if x as usize >= bounds_x || y as usize >= bounds_y {
        return None;
    }
    Some((x as usize, y as usize))
}