fn rounders(mut n: i32) -> i32 {
    let mut zeros = 0;
    while n >= 10 {
        let ones = n % 10;
        n /= 10;
        zeros += 1;
        if ones >= 5 {
            n += 1;
        }
    }
    return n * 10i32.pow(zeros);
}
