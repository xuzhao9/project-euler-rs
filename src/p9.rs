pub fn solve() {
    for i in (1..1000) {
        for j in (i..1000) {
            if (i * j ) % 1000 == 0 {
                if (i * j) / 1000  + 500 == (i + j) {
                    println!("i={}, j={}", i, j);
                    println!("{}", i * j * (1000 - i - j));
                }
            }
        }
    }
}
