fn additionWithoutCarrying(mut param1: i32, mut param2: i32) -> i32 {
    let mut result = 0;
    let mut iteration = 0;
    while param1 != 0 || param2 != 0 {
        result +=  10i32.pow(iteration) * ((param1 % 10 + param2 % 10) % 10);
        param1 /= 10;
        param2 /= 10;
        iteration += 1;
    }
    return result;
}
