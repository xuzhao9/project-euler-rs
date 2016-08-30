// Author: Xu Zhao (i@xuzhao.net)
// Date: Aug 29, 2016

//! problem 39

fn find_solutions(p: i32) -> i32 {
  let mut s = 0;
  for a in 1..p {
    for b in 1..p {
      if a + b < p {
        if p - a - b > a && p - a - b > b {
          let c = p - a - b;
          if a + b > c {
            if a * a  + b * b == c * c {
              s += 1;
            }
          }
        }
      }
    }
  }
  return s;
}

pub fn solve() {
  let mut sols = 0;
  let mut x = 0;
  for p in 12..1001 {
    if sols < find_solutions(p) {
      sols = find_solutions(p);
      x = p;
    }
  }
  println!("{}, {}", sols, x);
}
