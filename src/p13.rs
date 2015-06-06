use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

pub fn solve() {
    let d_path = format!("{}/fun/data.txt", env!{"HOME"});
    let file = File::open(d_path).ok().unwrap();
    let buf_reader = BufReader::new(file);
    let mut matrix = Vec::<Vec<u8>>::new();
    let mut result = Vec::<u8>::new();
    for l in buf_reader.lines() {
        let s: String = l.ok().unwrap();
        let mut bytes = s.into_bytes();
        bytes.reverse();
        matrix.push(bytes);
    }
    // for x in matrix {
    //     println!("{:?}", x);
    // }
    let mut carry = 0;
    for j in 0..matrix[0].len() {
        let mut sum: u64 = 0;
        for i in 0..matrix.len() {
            let digit = (matrix[i][j] - 48 ) as u64;
            sum = sum + digit;
        }
        result.push(((sum + carry) % 10) as u8 + 48);
        carry = (sum + carry) / 10;
    }
    // result.push(carry);
    result.reverse();
    let res = String::from_utf8(result);
    println!("{}{}", carry, res.unwrap());
}
