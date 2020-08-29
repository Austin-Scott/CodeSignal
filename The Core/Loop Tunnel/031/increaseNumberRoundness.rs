fn increaseNumberRoundness(mut n: i32) -> bool {
    while n % 10 == 0 {
        n /= 10;
    }
    n /= 10;
    while n > 0 {
        if n % 10 == 0 {
            return true;
        }
        n /= 10;
    }
    return false;
}
