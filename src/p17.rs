pub fn solve() {
    let mut digit = Vec::<u64>::new();
    for i in (0..21) {
        digit.push(4);
        digit.push(3);
        digit.push(3);
        digit.push(5);
        digit.push(4);
        digit.push(4);
        digit.push(3);
        digit.push(5);
        digit.push(5);
        digit.push(4);
        digit.push(3);
        digit.push(6); // eleven
        digit.push(6); // twelve
        digit.push(8); // thirteen
        digit.push(8); // fourteen
        digit.push(7); // fifteen
        digit.push(7); // sixteen
        digit.push(9); // seventeen
        digit.push(8); // eighteen
        digit.push(7); // ninteen
        digit.push(6); // twenty
    }
    for i in (21.. 1001) {
    }
    let mut sum = 0;
    for i in (1.. 1001) {
        sum += digit[i];
    }
    println!("sum={}", sum);
}
