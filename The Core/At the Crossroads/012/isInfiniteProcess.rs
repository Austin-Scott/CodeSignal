fn isInfiniteProcess(a: i32, b: i32) -> bool {
    if a > b { return true; }

    return a % 2 != b % 2;
}
