// Author: Xu Zhao (i@xuzhao.net)
// Date: Aug 30, 2016

//! problem 44

fn cal(n: i64) -> i64 {
  n * (3 * n - 1) / 2
}

fn is_pentag(n: i64) -> bool {
  let t = 24 * n + 1;
  let m = (t as f64).sqrt() as i64;
  if m * m == t && (m + 1) % 6 == 0 {
    return true;
  }
  return false;
}

pub fn solve() {
  let mut max = -1;
  let mut v: Vec<i64> = Vec::new();
  let mut index = 1;
  loop {
    let cur = cal(index);
    for i in 0..v.len() {
      if is_pentag(cur - v[i]) &&
        is_pentag(cur + v[i]) {
          if max == -1 || max > (cur - v[i]) {
            max = cur - v[i];
          }
        }
    }
    if index > 2 && max != -1 && max < (cur - v[v.len() - 1]) {
      break;
    }
    v.push(cur);
    index += 1;
  }
  println!("max={}", max);
}
