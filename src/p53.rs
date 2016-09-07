// Author: Xu Zhao (i@xuzhao.net)
// Date: Sep  6, 2016

//! p53

fn cal_comb(n: i64, r: i64) -> bool {
  let mut x = 1;
  println!("{}, {}", n, r);
  for p in (r+1)..n {
    x = x * p;
  }
  for p in 1..(n-r) {
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
    for r in 1..n {
      if cal_comb(n, r) {
        if n % 2 == 0 {
          cnt += 2 * (n / 2 - r + 1);
        } else {
          cnt += 2 * (n / 2 - r + 1) + 1;
        }
        break;
      }
    }
  }
  println!("{}", cnt);
}

