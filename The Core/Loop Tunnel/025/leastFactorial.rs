fn leastFactorial(n: i32) -> i32 {
    let mut factorial = 1;
    let mut multiplier = 2;
    loop {
        if factorial >= n {
            return factorial;
        }
        factorial *= multiplier;
        multiplier+=1;
    }
    return -1;
}
