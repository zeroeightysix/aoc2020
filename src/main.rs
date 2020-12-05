#![feature(try_trait)]

use std::ops::Range;
use std::option::NoneError;
use std::collections::HashSet;

fn main() -> Result<(), NoneError>{
    const INPUT: &str = include_str!("input");

    let seats: HashSet<i32> = INPUT.lines().map(|seat_code| {
        let mut row = (0..128);
        close_in(&mut row, 'F', 'B', &seat_code[0..7]);
        let mut seat = (0..8);
        close_in(&mut seat, 'L', 'R', &seat_code[7..10]);
        let row = row.start;
        let seat = seat.start;

        row * 8 + seat
    }).collect();

    let max = *seats.iter().max()?;
    println!("Highest seat code: {}", max);

    println!("Missing seats: {:?}", (0..=max).filter(|c| {
        !seats.contains(&c)
    }).collect::<Vec<i32>>());

    Ok(())
}

fn close_in(range: &mut Range<i32>, lower_char: char, upper_char: char, input: &str) {
    input.chars().for_each(|c| {
        let half = range.start + (range.end - range.start) / 2;
        match c {
            _ if c == lower_char => range.end = half,
            _ if c == upper_char => range.start = half,
            _ => panic!("char not F or B"),
        }
    })
}
