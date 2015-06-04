extern crate primes;

pub fn solve() {
    let mut prime_set = primes::PrimeSet::new();
    println!("{}", prime_set.get(10000));
}
