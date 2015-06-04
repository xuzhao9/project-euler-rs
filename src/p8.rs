use std::fs::File;
use std::io::Read;

fn multiply(buf: &Vec<u8>, index: usize) -> usize {
    let mut res: u64 = 1;
    for i in 0..13 {
        res = res * ((buf[i + index] - 48) as u64);
    }
    res as usize
}


pub fn solve() {
    let d_path = format!("{}/fun/data.txt", env!("HOME"));
    let mut file = match File::open(d_path) {
        Ok(file) => file,
        Err(..) => panic!("cannot open file"),
    };
    let mut buf = Vec::<u8>::new();
    file.read_to_end(&mut buf).ok();
    let mut index = 0;
    let mut result = 0;
    let mut has_zero = false;
    for &i in buf.iter() {
        if i >= 48 {
            if index >= 12 {
                if multiply(&buf, index - 12) > result {
                    result = multiply(&buf, index - 12);
                }
            }
            index = index + 1;
        }
    }
    println!("{}", result);
}
