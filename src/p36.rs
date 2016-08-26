// Author: Xu Zhao (i@xuzhao.net)
// Date: Aug 26, 2016

//! problem 36

fn num_to_vecdigits(n: i32, base: i32) -> Vec<i32> {
  let mut v: Vec<i32> = Vec::new();
  let mut nn = n.clone();
  loop {
    if nn == 0 {
      break;
    }
    v.push(nn % base);
    nn = nn / base;
  }
  return v;
}

fn is_palindromic(v: Vec<i32>) -> bool {
  let mut v_copy = v.clone();
  v_copy.reverse();
  for i in 0..v.len() {
    if v[i] != v_copy[i] {
      return false;
    }
  }
  return true;
}

pub fn solve() {
  let mut sum = 0;
  for n in 1..1000000 {
    if is_palindromic(num_to_vecdigits(n, 10)) &&
      is_palindromic(num_to_vecdigits(n, 2)) {
        sum += n;
      }
  }
  println!("{}", sum);
}

