mod test {
    use crate::solve::{solve_t1, solve_t2};

    static INPUT: &str = "2333133121414131402";

    #[test]
    fn sample_t1() {
        let res = solve_t1(&INPUT).unwrap();

        assert_eq!(1928, res);
    }

    #[test]
    fn sample_t2() {
        let res = solve_t2(&INPUT).unwrap();

        assert_eq!(2858, res);
    } 

}
