// Author: Xu Zhao (i@xuzhao.net)
// Date: Sep  6, 2016

//! p53

use std::cmp;

fn cal_comb(n: i64, r: i64) -> bool {
  if r == 0 || r == n {
    return false;
  }
  let mut x = 1;
  let a = cmp::min(r, n-r);
  let b = cmp::max(r, n-r);
  for p in (b+1)..(n+1) {
    x = x * p;
  }
  for p in 1..(a+1) {
    x = x / p;
  }
  if x > 1000000 {
    return true;
  }
  return false;
}

pub fn solve() {
  let mut cnt = 0;
  for n in 1..101 {
    for r in 0..n+1 {
      if cal_comb(n, r) {
        if n % 2 == 0 {
          cnt += n + 1 - r * 2;
        } else {
          cnt += n + 1 - r * 2;
        }
        break;
      }
    }
  }
  println!("{}", cnt);
}

