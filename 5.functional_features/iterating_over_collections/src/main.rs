use std::collections::HashMap;

/// `scores.iter()` --> Returns an iterator with immutable references to items, used in for loop as `in &scores`.
/// `scores.iter_mut()` --> Returns an iterator with mutable references to items, used in for loop as `in &mut scores`.
/// `scores.into_iter()` --> Returns an iterator with owned references to items, used in for loop as `in scores`.
fn main() {
    let mut scores = HashMap::new();
    scores.insert("red team".to_string(), 2);
    scores.insert("blue team".to_string(), 8);
    scores.insert("green team".to_string(), 6);

    let mut score_iter = scores.iter();
    let first: Option<(&String, &i32)> = score_iter.next();

    println!("{first:?}");

    for (team, score) in &scores {
        println!("{team} Got: {score} points");
    }

    let mut score_iter = scores.iter_mut();
    let first: Option<(&String, &mut i32)> = score_iter.next();

    println!("{first:?}");

    for (team, score) in &mut scores {
        *score += 1;
        println!("{team} Got: {score} points");
    }

    // let score_iter = scores.into_iter();
    // let first: Option<(String, i32)> = score_iter.next();

    for (team, score) in scores {
        println!("{team} Got {score} points");
    }
}
