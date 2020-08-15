fn maxMultiple(divisor: i32, bound: i32) -> i32 {
    for i in (1..=bound).rev() {
        if i % divisor == 0 {
            return i;
        }
    }
    return 1;
}