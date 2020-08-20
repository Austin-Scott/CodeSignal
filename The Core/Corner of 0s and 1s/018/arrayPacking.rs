fn arrayPacking(a: Vec<i32>) -> i32 {
    return a.iter().rev().fold(0, |acc, x| (acc<<8)|x);
}
