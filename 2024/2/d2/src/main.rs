use std::{fmt::Error, fs, process::exit};

fn main() {
    let input = load_file("./input");

    let one = solve_t1(&input.clone());

    println!("{one}");

    let two = solve_t2(&input);

    println!("{two}");

}

fn load_file(path: &str) -> String {
    let file= fs::read_to_string(path);
    match file {
        Ok(x) => {
            x
        },
        Err(_) => {
            print!("cant open file");
            exit(1)
        }
    }
}

fn solve_t1(input: &str) -> i32 {
    let it = input.split("\n");

    let mut count = 0;
    for i in it {
        let items: Vec<i32> = i.split(" ").map(|x| x.parse::<i32>().unwrap()).collect();
        let ascending = items.get(0) < items.get(1);
        let mut poison = false;
        for j in 0..items.len(){
            if j + 1 < items.len() {
                let cur = items.get(j).unwrap();
                let next = items.get(j+1).unwrap();
                if !(ascending == (cur < next)) {
                    poison = true;
                    break;
                } else if (cur - next).abs() > 3 || (cur - next).abs() < 1 {
                    poison = true;
                    break;
                }
            }
        }
        if !poison{
            count += 1;
        }
    }

    return count;
}

fn solve_t2(input: &str) -> i32 {
    let lines: Vec<Vec<i32>> = input
        .lines()
        .map(|x| {
            x.split(" ")
                .map(|n| n.parse::<i32>().unwrap())
                .collect()
        }).collect();

    let mut count = 0;
    for line in lines {
        match find_error_index(&line) {
            Some(_) => { 
                for j in 0..line.len() {
                    let mut cpy = line.clone();
                    cpy.remove(j);
    
                    if find_error_index(&cpy).is_none() {
                        count += 1;
                        break;
                    } 
                }
            },
            None => {
                count += 1;
            }
        }
    }
    
    return count;
}

fn find_error_index(items: &Vec<i32>) -> Option<usize> {
    let ascending = items.get(0)? < items.get(1)?;
    for i in 0..items.len() {
        if i+1 >= items.len() {continue;}
        
        let cur = items.get(i)?;
        let next = items.get(i+1)?;
        
        if !check_items(&ascending, cur, next) {
            return Some(i);
        }
    }

    return None;
}

fn check_items(ascending: &bool, cur: &i32, next: &i32) -> bool {
    if !(*ascending == (cur < next)) {
        return false;
    } else if (cur - next).abs() > 3 || (cur - next).abs() < 1 {
        return false;
    }
    return true;
}


// testin to live and sample input
#[cfg(test)]
mod tests {
    use crate::{load_file, solve_t2, solve_t1};


    static input: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn sample_t1() {
        let solve = solve_t1(input);
        assert_eq!(solve, 2);
    }

    #[test]
    fn data_t1() {
        let inp = load_file("./input");
        let solve = solve_t1(&inp);
        assert_eq!(solve, 479);
    }

    #[test]
    fn sample_t2() {
        let solve = solve_t2(input);
        assert_eq!(solve, 4);
    }

    #[test]
    fn data_t2() {
        let inp = load_file("./input");
        let solve = solve_t2(&inp);
        assert_eq!(solve, 531);
    }
    

}