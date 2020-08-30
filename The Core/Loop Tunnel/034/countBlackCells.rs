fn gcd(a: &i32, b: &i32) -> i32 {
    let min = std::cmp::min(a, b);
    let mut result = 1;
    for i in 1..=*min {
        if a % i == 0 && b % i == 0 {
            result = i;
        }
    }
    return result;
}
fn countBlackCells(n: i32, m: i32) -> i32 {
    let d = gcd(&n, &m);
    let mut p = 0;
    if d > 1 {
        p = (2*d)-2;
    }
    return (n + m - d) + p;
}
