fn reachNextLevel(experience: i32, threshold: i32, reward: i32) -> bool {
    return experience + reward >= threshold;
}