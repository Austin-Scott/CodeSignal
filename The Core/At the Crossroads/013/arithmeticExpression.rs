fn arithmeticExpression(a: i32, b: i32, c: i32) -> bool {
    let division = if a % b == 0 { a / b } else { -1 };
    return [a+b, a-b, a*b, division].contains(&c);
}
