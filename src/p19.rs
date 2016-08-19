// Author: Xu Zhao (i@xuzhao.net)
// Date: Aug 19, 2016

//! Problem 19

fn get_day(year: i32, mon: i32) -> i32 {
    if mon == 2 {
        if year % 4 == 0 {
            if year % 100 == 0 && year % 400 != 0 {
                28
            } else {
                29
            }
        } else {
            28
        }
    } else {
        let arr: [i32; 13] = [0, 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        arr[mon as usize]
    }
}

pub fn solve() {
    let mut x = 1; // 0 for Sunday, x for the first day in the month
    let mut r = 0;
    for year in 1900..2001 {
        for mon in 1..13 {
            x += get_day(year, mon);
            x = x % 7;
            if x == 0 && year > 1900 {
                r += 1;
            }
        }
    }
    println!("r={}", r);
}

