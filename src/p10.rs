extern crate primes;

pub fn solve() {
    let mut prime_set = primes::PrimeSet::new();
    let m = prime_set.iter().take_while(|&x| x <= 2000000u64)
        .fold(0, |acc, x| {acc + x});
    println!("{}", m);
}
