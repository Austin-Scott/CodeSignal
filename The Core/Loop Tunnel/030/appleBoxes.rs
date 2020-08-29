fn appleBoxes(k: i32) -> i32 {
    let mut redApples = 0;
    let mut yellowApples = 0;
    for i in 1..=k {
        if i % 2 == 0 {
            redApples += i*i;
        } else {
            yellowApples += i*i;
        }
    }
    return redApples - yellowApples;
}
