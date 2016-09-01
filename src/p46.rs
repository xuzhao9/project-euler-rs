// Author: Xu Zhao (i@xuzhao.net)
// Date: Sep  1, 2016

//! problem 46

extern crate primes;

fn is_square(n: u64) -> bool {
  let sq = (n as f64).sqrt() as u64;
  if sq * sq == n {
    return true;
  }
  false
}

fn is_want(n: u64, primes: &Vec<u64>)  -> bool{
  for p in primes {
    if *p >= n {
      return true;
    } else {
      if *p != 2 && is_square((n - *p) / 2) {
        return false;
      }
    }
  }
  return true;
}

pub fn solve() {
  let mut n = 33;
  let mut pset = primes::PrimeSet::new();
  let mut primes: Vec<u64> = Vec::new();
  let mut piter = pset.iter();
  loop {
    while primes.len() == 0 || primes[primes.len() - 1] < n {
      primes.push(piter.next().unwrap());
    }
    if !primes::is_prime(n) && is_want(n, &primes) {
      println!("{}", n);
      break;
    }
    n += 2;
  }
}

