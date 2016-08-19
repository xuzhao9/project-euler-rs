// Author: Xu Zhao (i@xuzhao.net)
// Date: Aug 19, 2016

//! Project Euler Problem 18

use std::fs::File;
use std::io::prelude::*;
use std::error::Error;
use std::path::Path;
use std::io::BufReader;

fn parse(s: String) -> Vec<i32> {
    let split = s.split(" ").collect::<Vec<&str>>();
    let mut r: Vec<i32> = Vec::new();
    for sp in split {
        let ss = String::from(sp);
        r.push(ss.parse::<i32>().ok().unwrap());
    }
    r
}

fn max(a: i32, b: i32) -> i32 {
    if a > b {
        a
    } else {
        b
    }
}

pub fn solve() {
    let mut state = [[0i32; 18]; 18];
    let mut line_cnt = 0;
    let data_path = Path::new("./data/p18.txt");
    let file = match File::open(&data_path) {
        Err(why) => panic!("couldn't open {}: {}", data_path.display(), why.description()),
        Ok(file) => file,
    };
    let reader = BufReader::new(file);
    let mut r = 0;
    for line in reader.lines() {
        let line_str = line.ok().unwrap();
        let numbers = parse(line_str);
        println!("count: {}", numbers.len());
        for j in 0..numbers.len() {
            if line_cnt == 0 {
                state[line_cnt][j] = numbers[j];
            } else {
                if j == 0 {
                    state[line_cnt][j] = max(state[line_cnt - 1][j], state[line_cnt - 1][j + 1]);
                } else {
                    state[line_cnt][j] = max(state[line_cnt - 1][j], state[line_cnt - 1][j - 1]);
                }
                state[line_cnt][j] += numbers[j];
            }
            println!("Val: ({},{}), {}", line_cnt, j, state[line_cnt][j]);
            if state[line_cnt][j] > r {
                r = state[line_cnt][j];
            }
        }
        line_cnt += 1;
    }
    println!("{}", r);
}


