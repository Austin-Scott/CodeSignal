fn lineUp(commands: String) -> i32 {
    let mut turns = 0;
    let mut result = 0;
    for c in commands.chars() {
        if(c == 'L' || c == 'R') {
            turns += 1;
        }
        if turns % 2 == 0 {
            result += 1;
        }
    }
    return result;
}
