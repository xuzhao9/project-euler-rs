// Author: Xu Zhao (i@xuzhao.net)
// Date: Sep  1, 2016

//! problem 49

extern crate primes;
use std::collections::HashMap;
use std::collections::HashSet;

fn digits_to_num(v: &Vec<i64>) -> i64 {
  let mut x = 0;
  for e in v {
    x = x * 10 + *e;
  }
  return x;
}

fn num_to_digits(num: i64) -> Vec<i64> {
  if num == 0 {
    return vec![0i64];
  }
  let mut nn = num.clone();
  let mut v: Vec<i64> = Vec::new();
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

pub fn solve() {
  let mut pset = primes::PrimeSet::new();
  let primes = pset.iter().take_while(|x| *x < 10000)
    .collect::<Vec<u64>>();
  let mut hset: HashMap<u64, HashSet<u64>> = HashMap::new();
  for p in primes.iter() {
    if *p > 1000 {
      let mut digits = num_to_digits(*p as i64);
      digits.sort();
      let num = digits_to_num(&digits);
      if num > 1000 {
        if !hset.contains_key(&(num as u64)) {
          hset.insert(num as u64, HashSet::new());
        }
        hset.get_mut(&(num as u64)).unwrap().insert(*p);
      }
   }
  }
  for k in hset.keys() {
    if hset.get(k).unwrap().len() >= 3 {
      for a in hset.get(k).unwrap() {
        for b in hset.get(k).unwrap() {
          if a != b && hset.get(k).unwrap().contains(&((a + b) / 2)) {
            println!("{}, {}, {}", a, (a+b)/2, b);
          }
        }
      }
    }
  }
}

