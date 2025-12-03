mod test {
    #[allow(unused_imports)]
    use crate::solve::{solve_t1, solve_t2};

    #[allow(dead_code)]
    static INPUT: &str = "987654321111111
811111111111119
234234234234278
818181911112111";

    #[test]
    fn sample_t1() {
        let res = solve_t1(&INPUT).unwrap();

        assert_eq!(357, res);
    }

    #[test]
    fn sample_t2() {
        let res = solve_t2(&INPUT).unwrap();

        assert_eq!(0, res);
    } 

}
