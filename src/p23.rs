// Author: Xu Zhao (i@xuzhao.net)
// Date: Aug 19, 2016

//! Problem 23

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
  let mut arr: [i32; 28124] = [0i32; 28124];
  let mut abun: Vec<i32> = Vec::new();
  for i in 1..28124 {
    let d = cal_d(i);
    if d > i {
      abun.push(i);
    }
  }
  for i in &abun {
    for j in &abun {
      if i + j < 28124 {
        arr[(i + j) as usize] = 1;
      }
    }
  }
  let mut sum = 0;
  for i in 1..28124 {
    if arr[i] == 0 {
      // println!("{}", i);
      sum += i;
    }
  }
  println!("{}", sum);
}

