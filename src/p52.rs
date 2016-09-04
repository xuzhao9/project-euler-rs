// Author: Xu Zhao (i@xuzhao.net)
// Date: Sep  3, 2016

//! p52

use std::collections::HashSet;

fn num_to_digits(num: i32) -> Vec<i32> {
  if num == 0 {
    return vec![0i32];
  }
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

fn get_digits(n: i32) -> Vec<HashSet<i32>> {
  let mut v: Vec<HashSet<i32>> = Vec::new();
  for i in 1..7 {
    let x = n * i;
    let set: HashSet<i32> = num_to_digits(x).iter().map(|&x| x).collect::<HashSet<i32>>();
    v.push(set);
  }
  v
}

fn all_equal_set(sets: Vec<HashSet<i32>>) -> bool {
  let mut s: HashSet<i32> = HashSet::new();
  for i in sets[0].iter() {
    s.insert(i.clone());
  }
  for ss in sets.iter() {
    if s.len() != ss.len() {
      return false;
    }
    for a in s.iter() {
      if !ss.contains(&a) {
        return false;
      }
    }
  }
  return true;
}

pub fn solve() {
  let mut x = 100;
  loop {
    let vv = get_digits(x);
    if all_equal_set(vv) {
      println!("{}", x);
      break;
    }
    x += 1;
  }
}

