// Author: Xu Zhao (i@xuzhao.net)
// Date: Aug 30, 2016

//! problem 43
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

// generate all combinations of 1-n
fn comb(elements: &Vec<i64>, start: usize, end: usize) -> Vec<Vec<i64>> {
  let e = elements[end];
  if start == end {
    return vec![vec![elements[start]]];
  }
  let small_comb = comb(elements, start, end - 1);
  let mut r: Vec<Vec<i64>> = Vec::new();
  for x in small_comb.iter() {
    for i in start..end+1 {
      let mut yy = x.clone();
      if i == end {
        yy.push(e);
      } else {
        yy.insert(i, e);
      }
      r.push(yy.clone());
    }
  }
  return r;
}

fn check(n: i64) -> bool {
  if n < 10i64.pow(9u32) {
    return false;
  }
  let digits = num_to_digits(n);
  let arr: [i64; 7] = [2, 3, 5, 7, 11, 13, 17];
  for i in 1..8 {
    let v = vec![digits[i], digits[i+1], digits[i+2]];
    let x = digits_to_num(&v);
    if x % arr[i - 1] != 0 {
      return false;
    }
  }
  true
}

pub fn solve() {
  let mut sum = 0;
  let elements = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
  for n in comb(&elements, 0, 9) {
    let num = digits_to_num(&n);
    if check(num) {
      println!("{}", num);
      sum += num;
    }
  }
  println!("{}", sum);
}

// 9012345678
