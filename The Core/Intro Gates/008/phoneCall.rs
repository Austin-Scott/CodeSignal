fn phoneCall(min1: i32, min2_10: i32, min11: i32, s: i32) -> i32 {
    let cost_of_first_ten_minutes = min2_10 * 9 + min1;
    if s >= cost_of_first_ten_minutes {
        let additional_funds = s - cost_of_first_ten_minutes;
        return additional_funds / min11 + 10;
    } else if s >= min1 {
        let additional_funds = s - min1;
        return additional_funds / min2_10 + 1;
    }
    return 0;
}