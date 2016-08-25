// Author: Xu Zhao (i@xuzhao.net)
// Date: Aug 25, 2016

//! problem 30

fn cal_sum(u: i32) -> i32 {
    let mut nums: Vec<i32> = Vec::new();
    let mut u_cp = u;
    let mut sum = 0;
    loop {
        if u_cp == 0 {
            break;
        } else {
            nums.push(u_cp % 10);
            sum += (u_cp % 10).pow(5);
            u_cp = u_cp / 10;
        }
    }
    return sum;
}

pub fn solve() {
    let mut sum = 0;
    for n in 10..1000000 {
        if cal_sum(n) == n {
            sum += n;
        }
    }
    println!("{}", sum);
}

