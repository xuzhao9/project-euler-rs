// Author: Xu Zhao (i@xuzhao.net)
// Date: Aug 19, 2016

//! Problem 21

use std::collections::HashSet;

fn cal_d(n: i32) -> i32 {
  let mut set: HashSet<i32> = HashSet::new();
  let up = (n as f32).sqrt() as i32 + 1;
  for i in 1..up {
    if n % i == 0 {
      if i != 1 {
        set.insert(n / i);
      }
      set.insert(i);
    }
  }
  let sum = set.iter().fold(0, |acc, &x| acc + x);
  sum
}

pub fn solve() {
  let mut sum = 0;
  for candid in 1..10000 {
    let d = cal_d(candid);
    // println!("{}, {}", candid, d);
    if cal_d(d) == candid && d != candid {
      sum += candid;
    }
  }
  println!("{}", sum);
}

