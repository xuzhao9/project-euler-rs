pub fn solve() {
    let mut res = 0;
    for i in (1..100) {
        let sum = (i+1 .. 101).fold(0, |acc, x| {acc + x});
        res = res + sum * i;
    }
    res = res * 2;
    println!("{}", res);
}
