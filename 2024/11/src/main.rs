use solution::solve::{solve_t1, solve_t2};

fn main() {
    let input = "5910927 0 1 47 261223 94788 545 7771".to_string();
    //let input = fs::read_to_string("./input").unwrap();
    println!("solution 1: {}", solve_t1(&input).unwrap());
    println!("solution 2: {}", solve_t2(&input).unwrap());
}
