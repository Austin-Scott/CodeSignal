fn magicalWell(mut a: i32, mut b: i32, mut n: i32) -> i32 {
    let mut sum = 0;
    while n > 0 {
        sum += a * b;
        a += 1;
        b += 1;
        n -= 1;
    }
    return sum;
}