use std::cmp::max;
use std::cmp::min;

fn tennisSet(score1: i32, score2: i32) -> bool {
    if (max(score1, score2) == 6 && min(score1, score2) != 5 ) || ( max(score1, score2) == 7 && [5, 6].contains(&min(score1, score2)) ) { return true; }
    return false;
}
