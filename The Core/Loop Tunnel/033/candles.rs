fn candles(mut candlesNumber: i32, makeNew: i32) -> i32 {
    let mut totalCandles = 0;
    let mut totalLeftovers = 0;
    while candlesNumber > 0 {
        // Burn all of the candles
        totalCandles += candlesNumber;
        totalLeftovers += candlesNumber;
        candlesNumber = 0;

        // Convert leftovers to new candles
        candlesNumber += totalLeftovers / makeNew;
        totalLeftovers = totalLeftovers % makeNew;
    }
    return totalCandles;
}
