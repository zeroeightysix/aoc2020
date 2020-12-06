use std::collections::HashSet;

fn main() {
    const INPUT: &str = include_str!("input");
    let s: usize = INPUT.split("\n\n")
        .map(count_combined)
        .sum();

    println!("Answer 1: {}", s);

    let i: usize = INPUT.split("\n\n")
        .map(count_intersection)
        .sum();

    println!("Answer 2: {}", i);
}

fn count_combined(lines: &str) -> usize {
    lines.chars()
        .filter(|c| !c.is_whitespace())
        .collect::<HashSet<char>>()
        .len()
}

fn count_intersection(lines: &str) -> usize {
    let mut iter = lines.lines().map(|l| l.chars().collect::<HashSet<char>>());
    let first = iter.next().unwrap();
    iter.fold(first, |acc, next| (&acc) & (&next)).len()
}