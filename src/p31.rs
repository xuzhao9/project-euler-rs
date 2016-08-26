// Author: Xu Zhao (i@xuzhao.net)
// Date: Aug 25, 2016

//! problem 31

fn find(amount: i32) -> i32 {
    let mut f: [[i32; 8]; 206] = [[0; 8]; 206];
    let arr: [usize; 8] = [1, 2, 5, 10, 20, 50, 100, 200];
    for i in 0..(amount+1) as usize {
        for j in 0..8 {
            if i == 0 {
                f[i][j] = 1;
            } else if i == 1 {
                f[i][0] = 1;
            } else {
                if i >= arr[j] {
                    if i == arr[j] {
                        f[i][j] = 1;
                    } else {
                        for k in 0..j+1 {
                            f[i][j] += f[i - arr[j]][k];
                        }
                    }
                }
            }
        }
    }
    let mut sum = 0;
    for j in 0..8 {
        sum += f[amount as usize][j];
    }
    return sum;
}

pub fn solve() {
    println!("{}", find(200));
}

