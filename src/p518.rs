extern crate primes;

fn prime_set( bool_array: &mut Vec<bool>, n: u64) -> Vec<u64> {
    let mut ret = Vec::<u64>::new();
    for _ in 0..n {
        bool_array.push(true);
    }
    bool_array[0] = false;
    bool_array[1] = false;
    let mut index = 2;
    loop {
        if index * index > n {
            break;
        } else {
            if bool_array[index as usize] {
                ret.push(index);
                let mut j = index * 2;
                while j < n {
                    bool_array[j as usize] = false;
                    j = j + index;
                }
            }
        }
        index += 1;
    }
    ret
}

pub fn solve() {
    let num = 100000000;
    let mut bool_array = Vec::<bool>::new();
    let p_vec = prime_set(&mut bool_array, num);
    println!("{}{}", "Done generaing primes, size:", p_vec.len());
    let len = p_vec.len();
    let mut sum = 0;
    for i in (0.. len - 2) {
        println!("{}/{}", i, len);
        for j in (i+1 .. len - 1) {

            let quar = (1+p_vec[j]) * (1+p_vec[j]);
            let first = 1 + p_vec[i];
            if quar > num * first {
                break;
            }
            if quar % first != 0 {
                continue;
            } else {
                let r = quar / first - 1;
                if bool_array[r as usize] && (r < num) && (r > p_vec[j]) {
                    // println!("{},{},{}", p_vec[j], p_vec[i], r);
                    sum += p_vec[i];
                    sum += p_vec[j];
                    sum += r;
                }
            }
        }
    }
    println!("{}" , sum);
}
