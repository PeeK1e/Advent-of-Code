mod test {
    #[allow(unused_imports)]
    use crate::solve::{solve_t1, solve_t2};

    #[allow(dead_code)]
    static INPUT: &str = "125 17";

    #[test]
    fn sample_t1() {
        let res = solve_t1(&INPUT).unwrap();

        assert_eq!(55312, res);
    }

    #[test]
    fn real_t1() {
        let input = "5910927 0 1 47 261223 94788 545 7771".to_string();
        let res = solve_t1(&input).unwrap();

        assert_eq!(193607, res);
    }


    #[test]
    fn sample_t2() {
        let res = solve_t2(&INPUT).unwrap();

        assert_eq!(0, res);
    } 

}
