// Author: Xu Zhao (i@xuzhao.net)
// Date: Aug 19, 2016

//! problem 26
extern crate primes;

fn find_k(n: &u64) -> i32 {
  let mut m: u64 = 10;
  let mut k = 1;
  loop {
    if m % n == 1 {
      return k;
    } else {
      m = 10 * (m % n);
      k += 1;
    }
  }
}

pub fn solve() {
  let mut t = 0;
  let mut d = 0;
  let mut prime_set = primes::PrimeSet::new();
  let m: Vec<u64> = prime_set.iter().take_while(|&x| x <= 1000).collect::<Vec<u64>>();
  for i in &m {
    if *i == 2 || *i == 5 {
      continue;
    }
    if find_k(i) > t {
      t = find_k(i);
      d = *i;
    }
    // println!("caling {}, {}", i, find_k(i));
  }
  println!("{}", d);
}

