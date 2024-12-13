mod test {
    use crate::solve::wirsing;
    #[allow(unused_imports)]
    use crate::solve::{solve_t1, solve_t2};

    #[allow(dead_code)]
    static INPUT: &str = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";

    #[test]
    fn sample_t1() {
        let res = solve_t1(&INPUT).unwrap();

        assert_eq!(480, res);
    }

    #[test]
    fn test_cramer_rule() {
        let res = wirsing((3,1,2), (6,4,5));
        
        assert_eq!((-1,2), res);
    }

    #[test]
    fn sample_t2() {
        let res = solve_t2(&INPUT).unwrap();

        assert_eq!(480, res);
    } 

}
