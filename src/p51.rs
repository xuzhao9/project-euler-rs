// Author: Xu Zhao (i@xuzhao.net)
// Date: Sep  2, 2016

//! p51

extern crate primes;

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

fn check_num(v: &Vec<i64>, a: usize, b: usize, c: usize) -> bool {
  let mut z = 0;
  let mut vv = v.clone();
  for n in 0..10 {
    vv[a] = n;
    vv[b] = n;
    vv[c] = n;
    let p = digits_to_num(&vv);
    if primes::is_prime(p as u64) && p > 100000 {
      z += 1;
    }
  }
  if z == 8 {
    return true;
  }
  false
}

pub fn solve() {
  let possible_digits: [i64; 3] = [0, 1, 2];
  // there must be three repetitive numbers
  // otherwise some of the eight numbers is dividable by three
  for i in 100..1000 {
    let digits = num_to_digits(i);
    for d in possible_digits.iter() {
      for a in 0..digits.len() {
        for b in a..digits.len() {
          for c in b..digits.len() {          
            let mut new_digits = digits.clone();
            new_digits.insert(a, *d);
            new_digits.insert(b+1, *d);
            new_digits.insert(c+2, *d);
            let new_num = digits_to_num(&new_digits);
            if new_num > 100000 && check_num(&new_digits, a, b+1, c+2) {
              println!("{}", new_num);
            }
          }
        }
      }
    }
  }
}

