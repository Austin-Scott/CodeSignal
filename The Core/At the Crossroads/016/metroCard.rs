fn metroCard(lastNumberOfDays: i32) -> Vec<i32> {
    if lastNumberOfDays == 30 { return vec![31]; }
    if lastNumberOfDays == 31 { return vec![28, 30, 31]; }
    return vec![31];
}
