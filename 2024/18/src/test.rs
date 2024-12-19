mod test {
    #[allow(unused_imports)]
    use crate::solve::{solve_t1, solve_t2};

    #[allow(dead_code)]
    static INPUT: &str = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";

    #[test]
    fn sample_t1() {
        let res = solve_t1(&INPUT, 7, 7, 12).unwrap();

        assert_eq!(22, res);
    }

    #[test]
    fn sample_t2() {
        let res = solve_t2(&INPUT, 7,7,12).unwrap();

        assert_eq!(res, (6,1));
    }
}
