fn addTwoDigits(n: i32) -> i32 {
    let s = n.to_string();
    let mut sum: i32 = 0;
    for c in s.chars() {
        sum += c.to_digit(10).unwrap() as i32;
    }
    return sum;
}
