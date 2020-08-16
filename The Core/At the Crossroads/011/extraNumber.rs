fn extraNumber(a: i32, b: i32, c: i32) -> i32 {
    if a == b {
        return c;
    } else if a == c {
        return b;
    } else {
        return a;
    }
}