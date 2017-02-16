// Author: Xu Zhao (i@xuzhao.net)
// Date: Feb 16, 2017

extern crate primes;

pub fn solve() {
  let mut v: Vec<u64> = Vec::new();
  v.push(1);
  let mut step = 2;
  let mut left = 4;
  let mut num_primes = 0;
  loop {
    let x = v.get(v.len() - 1).unwrap().clone();
    if primes::is_prime(x + step) {
      num_primes += 1;
    }
    v.push(x + step);
    left -= 1;
    if left == 0 {
      let xx = (num_primes as f64) / (v.len() as f64); 
      left = 4;
      step += 2;
      // weird here, since at 26239 it is 5248 / 52481
      if xx < 0.1 {
        break;
      }
    }
  }
  println!("{}", step-1);
}

