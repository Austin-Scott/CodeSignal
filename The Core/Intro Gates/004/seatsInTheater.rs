fn seatsInTheater(nCols: i32, nRows: i32, col: i32, row: i32) -> i32 {
    let horizontal_distance_from_exit = (nCols - col) + 1;
    let vertical_distance_from_exit = nRows - row;
    return horizontal_distance_from_exit * vertical_distance_from_exit;
}