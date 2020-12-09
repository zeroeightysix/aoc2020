#![feature(iter_advance_by)]
#![feature(label_break_value)]

use std::collections::VecDeque;
use std::process::exit;

const PREAMBLE_LEN: usize = 25;

fn main() {
    const INPUT: &str = include_str!("input");

    let mut preamble: VecDeque<usize> = INPUT.lines().take(PREAMBLE_LEN).map(|c| {
        c.parse::<usize>().unwrap()
    }).collect();

    let mut input = INPUT.lines();
    let _ = input.advance_by(PREAMBLE_LEN);

    let invalid = 'inv: {
        for x in input {
            let number: usize = x.parse().unwrap();
            if is_valid(number, &preamble) {
                let _ = preamble.pop_front();
                preamble.push_back(number);
            } else {
                break 'inv Some(number);
            }
        }

        None
    }.unwrap();

    let v: Vec<usize> = INPUT.lines().map(|c| c.parse::<usize>().unwrap()).collect();
    let len = v.len();
    (0..len - 1).for_each(|start_index| {
        let mut index = start_index;
        let mut acc = 0;
        loop {
            acc += &v[index];
            if acc > invalid || index == len - 1 {
                break;
            } else if acc == invalid {
                let sub = &v[start_index..=index];
                let min = sub.iter().min().unwrap();
                let max = sub.iter().max().unwrap();
                println!("Found weakness: {} to {}; min {} + max {} = {}", start_index, index, min, max, min + max);
                exit(0);
            }
            index += 1;
        }
    })
}

#[inline]
fn is_valid(number: usize, preamble: &VecDeque<usize>) -> bool {
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