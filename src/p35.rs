// Author: Xu Zhao (i@xuzhao.net)
// Date: Aug 26, 2016

//! problem 35

extern crate primes;
use std::collections::HashSet;

fn num_to_digits(num: u64) -> Vec<u64> {
  let mut nn = num.clone();
  let mut v: Vec<u64> = Vec::new();
  loop {
    if nn == 0 {
      v.reverse();
      return v;
    }
    let d = nn % 10;
    v.push(d);
    nn = nn / 10;
  }
}

fn find_cycles(num: u64) -> Vec<u64> {
  let digits = num_to_digits(num);
  let mut v: Vec<u64> = Vec::new();
  for start_index in 0..digits.len() {
    let mut new_number = 0;
    for traverse_index in start_index..start_index+digits.len() {
      new_number = new_number * 10 + digits[traverse_index % digits.len()];
    }
    v.push(new_number);
  }
  return v;
}

pub fn solve() {
  let mut prime_set = primes::PrimeSet::new();
  let prime_1million: HashSet<u64> = prime_set.iter().take_while(|&x| x<=1000000)
    .collect::<HashSet<u64>>();
  let mut t = 0;
  for prime in prime_1million.iter() {
    let cycles = find_cycles(prime.clone());
    let mut flag = true;
    for c in cycles {
      if !prime_1million.contains(&c) {
        flag = false;
        break;
      }
    }
    if flag {
      t += 1;
    }
  }
  println!("{}", t);
}

