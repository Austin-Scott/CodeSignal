fn lateRide(n: i32) -> i32 {
    let hours = n / 60;
    let minutes = n % 60;
    return (hours / 10) + (hours % 10) + (minutes / 10) + (minutes % 10);
}