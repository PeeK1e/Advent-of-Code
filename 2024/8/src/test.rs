mod test {
    use std::fs;

    use crate::solve::{solve_t1, solve_t2};

    static INPUT: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    #[test]
    fn sample_t1() {
        // example is off by one or i cant count
        // anyway, the solution is correct
        let res = solve_t1(&INPUT).unwrap();

        assert_eq!(14, res);
    }

    #[test]
    fn sample_t2() {
        let res = solve_t2(&INPUT).unwrap();

        assert_eq!(34, res);
    } 

}
