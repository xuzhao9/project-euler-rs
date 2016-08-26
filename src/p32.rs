// Author: Xu Zhao (i@xuzhao.net)
// Date: Aug 25, 2016

//! problem 32

use std::collections::HashSet;

fn has_duplicate(arr: &Vec<i32>) -> bool {
  if arr.len() == 0 {
    return true;
  }
  let mut b: [bool; 10] = [false; 10];
  for x in arr {
    if *x == 0 {
      return true;
    }
    if !b[*x as usize] {
      b[*x as usize] = true;
    } else {
      return true;
    }
  }
  false
}

fn get_digits(num: i32, arr: &mut Vec<i32>) {
  if num >= 10000 {
    return;
  }
  arr.push(num / 1000);
  arr.push(num % 1000 / 100);
  arr.push(num % 100 / 10);
  arr.push(num % 10);
}

fn check_pandigital(arr: &Vec<i32>) -> i32 {
  let m1 = 10 * arr[0] + arr[1];
  let m2 = 100 * arr[2] + 10 * arr[3] + arr[4];
  let m1pm2 = m1 * m2;
  let mut aa1 = arr.clone();
  get_digits(m1pm2, &mut aa1);
  if !has_duplicate(&aa1) && m1pm2 < 10000 {
    return m1pm2;
  }
  let m3 = arr[0];
  let m4 = 1000 * arr[1] + 100 * arr[2] + 10 * arr[3] + arr[4];
  let m3pm4 = m3 * m4;
  aa1 = arr.clone();
  get_digits(m3pm4, &mut aa1);
  if !has_duplicate(&aa1) && m3pm4 < 10000 {
    return m3pm4;
  }
  return 0;
}

pub fn solve() {
  let mut set: HashSet<i32> = HashSet::new();
  for d1 in 1..10 {
    for d2 in 1..10 {
      for d3 in 1..10 {
        for d4 in 1..10 {
          for d5 in 1..10 {
            let arr: Vec<i32> = vec![d1, d2, d3, d4, d5];
            if !has_duplicate(&arr) {
              let p = check_pandigital(&arr);
              if p != 0 {
                set.insert(p);
              }
            }
          }
        }
      }
    }
  }
  let sum = set.iter().fold(0, |sum, &x| sum + x);
  println!("{}", sum);
}

