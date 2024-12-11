use regex::Regex;

fn main() {
    let content = std::fs::read_to_string("./input").unwrap();

    let one = solve_t1(&content);
    println!("{one}");

    let two = solve_t2(&content);
    println!("{two}");
}

fn solve_t1(content: &str) -> i32 {
    let cap = Regex::new(r"(mul\([0-9]*,[0-9]*\))").unwrap();
    let mut count = 0;
    for (_, [mul]) in cap.captures_iter(content).map(|c| c.extract()) {
        let mut res = mul.replace("mul(", "");
        res = res.replace(")", "");
        
        let e = res.split(",").collect::<Vec<&str>>();
        let l = e[0].parse::<i32>().unwrap();
        let r = e[1].parse::<i32>().unwrap();
        count+= l*r;
    }

    count
}

fn solve_t2(content: &str) -> i32 {
    let cap = Regex::new(r"(mul\([0-9]*,[0-9]*\)|do\(\)|don't\(\))").unwrap();
    let mut count = 0;
    let mut do_task = true;
    for (_, [mul]) in cap.captures_iter(content).map(|c| c.extract()) {
        if mul.contains("do()") {
            do_task = true;
            continue;
        } else if mul.contains("don't()"){
            do_task = false;
            continue;
        }

        if !do_task {
            continue;
        }

        let mut res = mul.replace("mul(", "");
        res = res.replace(")", "");
        
        let e = res.split(",").collect::<Vec<&str>>();
        let l = e[0].parse::<i32>().unwrap();
        let r = e[1].parse::<i32>().unwrap();
        count+= l*r;
    }

    count
}

#[cfg(test)]
mod test {
    use crate::{solve_t1, solve_t2};

    static input: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    static input2: &str = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn sample_t1() {
        let res = solve_t1(&input);

        assert_eq!(161, res);
    }

    #[test]
    fn sample_t2() {
        let res = solve_t2(&input2);

        assert_eq!(48, res);
    }
}