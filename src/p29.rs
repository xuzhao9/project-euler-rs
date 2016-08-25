// Author: Xu Zhao (i@xuzhao.net)
// Date: Aug 25, 2016

//! problem 29
extern crate primes;
use std::collections::HashSet;

fn find_prime_fact_cnt(n: i32, p: i32) -> i32 {
    let mut i = 0;
    let mut n_cp = n;
    loop {
        if n_cp % p == 0 {
            i += 1;
            n_cp = n_cp / p;
        } else {
            return i;
        }
    }
}

fn find_prime_factorize(a: i32, b: i32) -> Vec<(i32, i32)> {
    let mut prime_set = primes::PrimeSet::new();
    let prime_100: Vec<u64> = prime_set.iter().take_while(|&x| x<= 100).collect::<Vec<u64>>();
    let mut r: Vec<(i32, i32)> = Vec::new();
    for p in prime_100 {
        if a % (p as i32) == 0 {
            let c = find_prime_fact_cnt(a, p as i32);
            r.push((p as i32, c * b));
        }
    }
    return r;
}

pub fn solve() {
    let mut hs: HashSet<Vec<(i32, i32)>> = HashSet::new();
    for a in 2..101 {
        for b in 2..101 {
            let set = find_prime_factorize(a, b);
            hs.insert(set);
        }
    }
    println!("Hs size: {}", hs.len());
}

