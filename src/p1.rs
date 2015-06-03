pub fn solve() -> i32 {
    let max_num = 1000;
    let numbers = (1.. max_num).filter(|&i| i % 3 == 0 || i % 5 == 0);
    numbers.fold(0, |acc, x| acc + x)
}

#[test]
fn p1_test() {
    let sum = solve();
    assert_eq!(sum, 233168);
}
