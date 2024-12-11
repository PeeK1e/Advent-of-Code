mod test {
    use std::fs;

    use crate::solve::{solve_t1, solve_t2};

    static INPUT: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn sample_t1() {
        let res = solve_t1(&INPUT).unwrap();

        assert_eq!(3749, res);
    }

    #[test]
    fn sample_t2() {
        let res = solve_t2(&INPUT).unwrap();

        assert_eq!(11387, res);
    } 

}
