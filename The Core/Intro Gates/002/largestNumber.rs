fn largestNumber(n: i32) -> i32 {
    let mut result = String::new();
    for i in 0..n {
        result.push('9');
    }
    return result.parse::<i32>().unwrap();
}