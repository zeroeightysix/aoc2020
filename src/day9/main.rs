#![feature(label_break_value)]

use std::process::exit;
use slice_deque::SliceDeque;

const PREAMBLE_LEN: usize = 25;

fn main() {
    let input: Vec<usize> = include_str!("input").lines().map(|c| c.parse::<usize>().unwrap()).collect();

    let invalid = 'inv: {
        let mut preamble: SliceDeque<usize> = SliceDeque::from(&input[0..=PREAMBLE_LEN]);
        let input = &input[PREAMBLE_LEN + 1..];

        for x in input {
            let x = *x;
            if is_valid(x, &preamble) {
                let _ = preamble.pop_front();
                preamble.push_back(x);
            } else {
                break 'inv x;
            }
        }

        unreachable!();
    };

    let len = input.len();
    let mut start_index = 0;
    loop {
        let mut index = start_index;
        let mut acc = 0;
        loop {
            acc += &input[index];
            if acc > invalid || index == len - 1 {
                break;
            } else if acc == invalid {
                let sub = &input[start_index..=index];
                let min = sub.iter().min().unwrap();
                let max = sub.iter().max().unwrap();
                println!("Found weakness: {} to {}; min {} + max {} = {}", start_index, index, min, max, min + max);
                exit(0);
            }
            index += 1;
        }

        start_index += 1;
    }
}

#[inline]
fn is_valid(number: usize, preamble: &SliceDeque<usize>) -> bool {
    for x in preamble {
        for y in preamble {
            if x == y {
                continue;
            }

            if x + y == number {
                return true;
            }
        }
    }

    return false;
}