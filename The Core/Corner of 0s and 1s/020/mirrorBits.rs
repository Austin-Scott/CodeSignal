fn mirrorBits(a: i32) -> i32 {
    let mut result = 0;
    let mut a = a;
    for i in 0..32 {
        result = result | (a % 2);
        a = a / 2;
        if a == 0 { break; }
        result = result * 2;
    }
    return result;
}
