use fibo;

pub fn solve() -> u32 {
    let m = fibo::fibo_seq()
        .filter(|&x| x % 2 == 0)
        .take_while(|&x| x <= 4000000u32)
        .fold(0, |acc, x| acc + x);
    m
}

#[test]
pub fn p2_test() {
    assert_eq!(solve(), 4613732);
}
