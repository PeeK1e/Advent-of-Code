mod test {
    #[allow(unused_imports)]
    use crate::solve::{solve_t1, solve_t2};

    #[allow(dead_code)]
    static INPUT: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    static EX_1: &str = "0090009
9991598
9992007
6543456
7650987
8760000
9870000";

    #[test]
    fn sample_t1() {
        let res = solve_t1(&INPUT).unwrap();
        println!("{res}");
        assert_eq!(36, res);
    }

    #[test]
    fn sample_t2() {
        let res = solve_t2(&INPUT).unwrap();

        assert_eq!(81, res);
    } 

}
