// Author: Xu Zhao (i@xuzhao.net)
// Date: Aug 20, 2016

//! problem 28 solution

pub fn solve() {
  let mut r = 1;
  let mut step = 2;
  let mut cur = 1;
  loop {
    if step > 1000 {
      break;
    } else {
      let arr: [i32;4] = [cur + step, cur + step * 2,
                        cur + step * 3, cur + step * 4];
      r = r + arr[0] + arr[1] + arr[2] + arr[3];
      cur = cur +  step * 4;
      step += 2;
    }
  }
  println!("{}", r);
}

