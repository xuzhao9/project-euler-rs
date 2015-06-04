use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::str::FromStr;
use std::cmp;

pub fn cal(matrix:& Vec<Vec<u64>>, i: usize, j: usize) -> u64 {
    let mut x = 0;
    let mut y = 0;
    let mut z = 0;
    if i + 4 <= matrix.len() {
        z = matrix[i][j] * matrix[i+1][j] * matrix[i+2][j] * matrix[i+3][j];
    }
    if j + 4 <= matrix[i].len() {
        y = matrix[i][j] * matrix[i][j+1] * matrix[i][j+2] * matrix[i][j+3];
    }
    if i + 4 <= matrix.len() && j + 4 <= matrix[i].len() {
        x = matrix[i][j] * matrix[i+1][j+1] * matrix[i+2][j+2] * matrix[i+3][j+3];
    }
    let mut k = 0;
    if i >= 3 && j + 4 <= matrix[i].len() {
        k = matrix[i][j] * matrix[i-1][j+1] * matrix[i-2][j+2] * matrix[i-3][j+3];
    }
    return cmp::max(cmp::max(cmp::max(x,y), z), k);
}

pub fn solve() {
    let d_path = format!("{}/fun/data.txt", env!("HOME"));
    let file = File::open(d_path).ok().unwrap();
    let buf_reader = BufReader::new(file);
    let mut matrix =Vec::<Vec<u64>>::new();
    let mut index = 0;
    for l in buf_reader.lines() {
        let s: String = l.ok().unwrap();
        matrix.push(Vec::<u64>::new());
        index += 1;
        let parts = s.split(" ");
        for p in parts {
            matrix[index - 1].push(u64::from_str(p).unwrap());
        }
    }
    let mut res = 0;
    // println!("len={}", matrix.len());
    for i in (0 .. matrix.len()) {
        for j in (0 .. matrix[i].len()) {
            if cal(& matrix, i, j) > res {
                res = cal(& matrix, i, j);
            }
        }
    }
    println!("{}", res);
}
