pub fn solve() {
    let mut s = 0;
    for i in 1..1001 {
        // println!("{}, {}", i, find_word(i));
        s += find_word(i);
    }
    println!("{}", s);
}

fn find_word(num: i32) -> i32 {
    // setup a table...
    let arr: [i32; 21] = [4, 3, 3, 5, 4, 4, 3, 5, 5, 4, 3, 6, 6,
                          8, 8, 7, 7, 9, 8, 8, 6];
    // THIRTEEN, FOURTEEN, FIFTEEN, SIXTEEN, SEVENTEEN,
    // EIGHTEEN, NINETEEN, TWENTY
    let arr2: [i32; 10] = [0, 0, 6, 6, 5, 5, 5, 7, 6, 6];
    // TWENTY, THIRTY, FORTY, FIFTY, SIXTY, SEVENTY, EIGHTY, NINETY
    if num == 1000 {
        11 // ONE THOUSAND
    } else if num < 21 && num > 0 {
        arr[num as usize]
    } else if num >= 100 {
        let hund = num / 100;
        let left = num % 100;
        match left {
            0 => arr[hund as usize] + 7, // HUNDRED
            _ => arr[hund as usize] + 10 + find_word(left), // HUNDRED AND
        }
    } else if num >= 21 && num < 100 {
        let ten = num / 10;
        let last = num % 10;
        match last {
            0 => arr2[ten as usize],
            _ => arr2[ten as usize] + arr[last as usize],
        }
    } else {
        0 // num == 0
    }
}
