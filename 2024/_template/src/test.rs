mod test {
    use std::fs;

    use crate::solve::{solve_t1, solve_t2};

    static INPUT: &str = "";

    #[test]
    fn sample_t1() {
        let res = solve_t1(&INPUT).unwrap();

        assert_eq!(0, res);
    }

    #[test]
    fn sample_t2() {
        let res = solve_t2(&INPUT).unwrap();

        assert_eq!(0, res);
    } 

}
