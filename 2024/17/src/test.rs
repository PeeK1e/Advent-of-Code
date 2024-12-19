mod test {
    #[allow(unused_imports)]
    use crate::solve::{solve_t1, solve_t2};

    #[allow(dead_code)]
    static INPUT: &str = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";

    static INPUT2: &str = "Register A: 2024
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";

    static INPUT3: &str = "Register A: 117440
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0";


    #[test]
    fn sample_t1() {
        let res = solve_t1(&INPUT).unwrap();
        assert_eq!("4,6,3,5,6,3,5,2,1,0", res);
    }

    #[test]
    fn sample_2_t1() {
        let res = solve_t1(&INPUT2).unwrap();
        assert_eq!("4,2,5,6,7,7,7,7,3,1,0", res);
    }
    #[test]
    fn sample_3_t1() {
        let res = solve_t1(&INPUT3).unwrap();
        assert_eq!("0,3,5,4,3,0", res);
    }

    #[test]
    fn sample_t2() {
        let res = solve_t2(&INPUT2).unwrap();

        assert_eq!("0,3,5,4,3,0", res);
    } 

}
