extern crate primes;

use std::collections::HashMap;

fn nth(n: i64) -> i64 {
    n * (n + 1) / 2
}

pub fn solve() {
    let mut i: i64 = 3;
    loop {
        let n = nth(i);
        // if n > 100 {break;}
        let factors = primes::factors(n as u64);
        let mut map = HashMap::<u64, u64>::new();
        for f in factors {
            if let Some(x) = map.get_mut(&f) {
                *x = *x + 1;
            }
            if !map.contains_key(&f) {
                map.insert(f, 1);
            }
        }
        let mut res = 1;
        for k in map.keys() {
            res = res * (map.get(&k).unwrap() + 1);
        }
        // println!("{:?}, res={}", map, res);
        if(res > 500) {
            println!("{}", n);
            break;
        }
        i += 1;
    }
}
