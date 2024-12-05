#[cfg(test)]
mod test {
    use crate::{solve_t1, solve_t2};

    static INPUT: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn sample_t1() {
        let res = solve_t1(&INPUT);

        assert_eq!(0, res);
        // assert_eq!(143, res);
    }

    #[test]
    fn sample_t2() {
        let res = solve_t2(&INPUT);

        assert_eq!(9, res);
    }
}

use std::collections::HashMap;
fn main() {
    let content = std::fs::read_to_string("./input").unwrap();

    let one: i32 = solve_t1(&content);
    println!("{one}");

    let two: i32 = solve_t2(&content);
    println!("{two}");
}

fn solve_t1(content: &str) -> i32 {
    let mut count = 0;
    let (map, order) = make_input_maps(&content);

    for o in order {
        for v in o.split(",") {
            
        }
    }

    return count;
}

fn solve_t2(content: &str) -> i32 {
    let mut count = 0;

    return count;
}

fn make_input_maps(content: &str) -> (HashMap<String, Vec<&str>>,Vec<&str>) {
    // split on instructions
    let split: Vec<&str> = content.split("\n\n").collect();
    // make usable maps
    let pages: Vec<&str> = split.get(0)
        .unwrap()
        .split("\n")
        .collect();
    let order: Vec<&str> = split.get(1)
        .unwrap()
        .split("\n")
        .collect();

    let mut map: HashMap<String, Vec<&str>> = HashMap::new();

    for e in pages {
        let pages: Vec<&str> = e.split("|").collect();
        let key = pages.get(0).unwrap().to_string().clone();
        let page = pages.get(1).unwrap();

        match map.get_mut(&key) {
            Some(v) => {
                v.push(page);
            }
            None => {
                let mut v = vec![];
                v.push(*page);
                map.insert(key, v);
            }
        }
    }

    return (map, order);
}