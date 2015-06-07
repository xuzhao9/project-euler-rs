// solved by pen and paper


pub fn solve() {
    // golden ratio
    let phi = ((5.0 as f64).sqrt()+1.0) / 2.0;
    let r = (999.0 * (10.0 as f64).log2() + (5.0 as f64).log2() / 2);
    println!("{}", r / phi.log2());
}
