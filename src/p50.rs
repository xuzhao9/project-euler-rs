// Author: Xu Zhao (i@xuzhao.net)
// Date: Sep  1, 2016

//! p50

extern crate primes;

pub fn solve() {
  let mut pset = primes::PrimeSet::new();
  let primes = pset.iter().take_while(|x| *x < 1000000).collect::<Vec<u64>>();
  let prime_sum = primes.iter().fold(vec![0], |mut acc, x|
                                     {
                                       let l = acc[acc.len() - 1];
                                       acc.push(l + x);
                                       acc
                                     });
  let mut ss = 0u64;
  let mut max_len = 0;
  for s in 0..prime_sum.len() {
    for e in s..prime_sum.len() {
      let to_test = prime_sum[e] - prime_sum[s];
      if to_test > 1000000 {
        break;
      }
      if primes::is_prime(to_test) && max_len < (e - s)
      && to_test < 1000000 {
        max_len = e - s;
        ss = to_test;
      }
    }
  }
  println!("{}", ss);
}

