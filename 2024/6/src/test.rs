mod test {
    use std::fs;

    use crate::solve::{solve_t1, solve_t2};

    static INPUT: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn sample_t1() {
        let res = solve_t1(INPUT).unwrap();

        assert_eq!(41, res);
    }

    #[test]
    fn sample_t2() {
        let res = solve_t2(&INPUT).unwrap();

        assert_eq!(6, res);
    } 

    #[test]
    fn real_t2() {
        let input = fs::read_to_string("./input").unwrap();
        let res = solve_t2(&input).unwrap();
        println!("solve: {}", res);
        //assert_eq!(6, res);
    }
}
