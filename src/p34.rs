// Author: Xu Zhao (i@xuzhao.net)
// Date: Aug 26, 2016

//! problem 34

fn digits(n: i32) -> Vec<i32> {
  let mut nn = n.clone();
  let mut v: Vec<i32> = Vec::new();
  loop {
    if nn == 0 {
      return v;
    }
    let d = nn % 10;
    v.push(d);
    nn = nn / 10;
  }
}

fn factorial(n: &i32) -> i32 {
  if *n == 1 || *n == 0 {
    1
  } else {
    *n * factorial(&(*n - 1))
  }
}

pub fn solve() {
  let mut sum = 0;
  for i in 10..1000000 {
    let ds = digits(i);
    let s = ds.iter().map(factorial).fold(0, |sum, x| sum + x);
    if s == i {
      sum += i;
    }
  }
  println!("{}", sum);
}

