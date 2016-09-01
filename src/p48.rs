// Author: Xu Zhao (i@xuzhao.net)
// Date: Sep  1, 2016

//! problem 48

// calculate last 10 digits of n^p
fn cal_product(n: u64, p: u64) -> u64 {
  let mut x = 1;
  for _ in 0..p {
    x = x * n;
    x = x % (10u64.pow(10));
  }
  x
}

// calcualte sum of all elements in argument
fn cal_add(v: Vec<u64>) -> u64 {
  v.iter().fold(0, |sum, x| sum + x) % (10u64.pow(10))
}

pub fn solve() {
  let x = cal_add((1..1001).map(|x| cal_product(x, x)).collect::<Vec<u64>>());
  println!("{}", x);
}

