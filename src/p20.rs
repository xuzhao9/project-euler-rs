// Author: Xu Zhao (i@xuzhao.net)
// Date: Aug 19, 2016

//! Problem 20
extern crate num;
use self::num::{BigUint, One};

fn fac(n: usize) -> BigUint {
    let mut f0: BigUint = One::one();
    let mut f1: BigUint = One::one();
    for _ in 2..n+1 {
        f1 = &f1 + &One::one();
        f0 = f0 * &f1;
    }
    f0
}

fn sum(num: BigUint) -> u32 {
    let mut s: String = num.to_str_radix(10);
    let mut r = 0;
    for c in s.as_str().chars() {
        let n = c.to_digit(10).unwrap();
        r += n;
    }
    return r;
}

pub fn solve() {
    println!("{}", sum(fac(100)));
}
