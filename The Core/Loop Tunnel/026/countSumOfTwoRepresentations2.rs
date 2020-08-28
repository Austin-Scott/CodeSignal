fn countSumOfTwoRepresentations2(n: i32, l: i32, r: i32) -> i32 {
    let mut sum = 0;
    for A in (l..=r) {
        for B in (A..=r) {
            if A + B == n {
                sum += 1;
            }
        }
    }
    return sum;
}
