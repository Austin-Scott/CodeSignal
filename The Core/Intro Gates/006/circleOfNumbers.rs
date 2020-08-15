fn circleOfNumbers(n: i32, firstNumber: i32) -> i32 {
    return (firstNumber + (n / 2)) % n;
}