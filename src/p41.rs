// Author: Xu Zhao (i@xuzhao.net)
// Date: Aug 29, 2016

//! problem 41

extern crate primes;

fn digits_to_num(v: &Vec<i32>) -> i32 {
  let mut x = 0;
  for e in v {
    x = x * 10 + *e;
  }
  return x;
}

fn num_to_digits(num: i32) -> Vec<i32> {
  let mut nn = num.clone();
  let mut v: Vec<i32> = Vec::new();
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

fn insert(v: &Vec<i32>, loc: usize, num: i32) -> Vec<i32> {
  let mut vv = v.clone();
  if loc == vv.len() {
    vv.push(num);
  } else {
    let mut x = vv[loc];
    vv.push(x);
    for i in loc+1..vv.len() {
      let t = vv[i];
      vv[i] = x;
      x = t;
    }
    vv[loc] = num;
  }
  return vv;
}

// generate all combinations of 1-n
fn comb(n: i32) -> Vec<i32> {
  if n == 1 {
    return vec![1i32];
  }
  let mut vv: Vec<i32> = Vec::new();
  let comb_n_1 = comb(n-1);
  for x in comb_n_1 {
    let digits = num_to_digits(x);
    for loc in 0..n as usize {
      let new_digits = insert(&digits, loc, n);
      vv.push(digits_to_num(&new_digits));
    }
  }
  return vv;
}

pub fn solve() {
  let mut max = 0;
  for n in 4..10 {
    for x in comb(n) {
      if primes::is_prime(x as u64) && x > max {
        max = x;
      }
    }
  }
  println!("{}", max);
}

