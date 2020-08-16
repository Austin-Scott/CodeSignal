fn knapsackLight(value1: i32, weight1: i32, value2: i32, weight2: i32, maxW: i32) -> i32 {
    if weight1 + weight2 <= maxW {
        return value1 + value2;
    } else if weight1 <= maxW && weight2 <= maxW {
        return if value1 > value2 { value1 } else { value2 };
    } else if weight1 <= maxW || weight2 <= maxW {
        return if weight1 > maxW { value2 } else { value1 };
    } else {
        return 0;
    }
}
