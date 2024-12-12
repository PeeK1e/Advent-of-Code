mod test {
    #[allow(unused_imports)]
    use crate::solve::{solve_t1, solve_t2};

    #[allow(dead_code)]
    static INPUT_SIMPLE: &str = "AAAA
BBCD
BBCC
EEEC";

    static INPUT_COMPLEX: &str = "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO";

    static INPUT_LARGE: &str = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

    #[test]
    fn sample_t1() {
        let res = solve_t1(&INPUT_SIMPLE).unwrap();

        assert_eq!(140, res);
    }
    #[test]
    fn sample_complex_t1() {
        let res = solve_t1(&INPUT_COMPLEX).unwrap();

        assert_eq!(772, res);
    }
    #[test]
    fn sample_large_t1() {
        let res = solve_t1(&INPUT_LARGE).unwrap();

        assert_eq!(1930, res);
    }

    #[test]
    fn sample_t2() {
        let res = solve_t2(&INPUT_SIMPLE).unwrap();

        //assert_eq!(0, res);
    } 

}