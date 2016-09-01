// Author: Xu Zhao (i@xuzhao.net)
// Date: Aug 31, 2016

//! problem 45

fn is_square(n: i64) -> bool {
  let sq = (n as f64).sqrt() as i64;
  if sq * sq == n {
    return true;
  }
  false
}

fn is_triangle(n: i64) -> bool {
  if is_square(1 + 8 * n) {
    return true;
  }
  false
}

fn is_hexa(n: i64) -> bool {
  // println!("{}",n);
  let sqrt = ((1 + 8 * n) as f32).sqrt() as i64;
  if is_square(1 + 8 * n) && ((1 + sqrt)%4 == 0) {
    return true;
  }
  false
}

pub fn solve() {
  let mut index = 166;
  loop {
    let t = index * (3 * index - 1) / 2;
    if is_hexa(t) && is_triangle(t) {
      println!("{}", t);
      break;
    }
    index += 1;
  }
}

