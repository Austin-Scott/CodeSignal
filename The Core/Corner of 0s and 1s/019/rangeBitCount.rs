fn sumBits(x: i32) -> i32 {
    let mut sum = 0;
    let mut x = x;
    for i in 0..32 {
        if x % 2 == 1 {
            sum += 1;
        }
        x = x / 2;
    }
    return sum;
}
fn rangeBitCount(a: i32, b: i32) -> i32 {
    if a == b { return sumBits(b); }
    return sumBits(a) + rangeBitCount(a+1, b);
}
