// Author: Xu Zhao (i@xuzhao.net)
// Date: Aug 27, 2016

//! problem 37

extern crate primes;

fn seq_to_num(seq: &Vec<i32>, start: usize, end: usize) -> i32 {
  let mut r = 0;
  for i in start..end + 1 {
    r = r * 10 + seq[i];
  }
  return r;
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

fn satisfy(num: i32) -> bool {
  let digits = num_to_digits(num);
  for i in 0..digits.len() {
    if !primes::is_prime(seq_to_num(&digits, i, digits.len() - 1) as u64) {
      //println!("{:?}, {}", digits, seq_to_num(&digits, i, digits.len() - 1));
      return false;
    }
  }
  for i in 0..digits.len() {
    if !primes::is_prime(seq_to_num(&digits, 0, i) as u64) {
      // println!("{}", seq_to_num(&digits, i, digits.len() - 1));
      return false;
    }
  }
  return true;
}

pub fn solve() {
  let mut num = 11;
  let mut cnt = 0;
  let mut r = 0;
  loop {
    if satisfy(num) {
      println!("num={}", num);
      cnt += 1;
      r += num;
    }
    if cnt == 11 {
      break;
    }
    num += 2;
  }
  println!("{}", r);
}

