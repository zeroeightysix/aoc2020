use std::collections::HashMap;
use std::time::Instant;

fn main() {
    const INPUT: &str = include_str!("input");

    let mut v: Vec<usize> = INPUT.lines().map(|c| c.parse::<usize>().unwrap()).collect();
    v.sort();

    let p1 = Instant::now();
    println!("Answer 1: {:?}, took {:?}", find_differences(&v), p1.elapsed());

    let p2 = Instant::now();
    let mut visited: HashMap<usize, usize> = HashMap::new();
    let internal = v.last().unwrap() + 3;
    v.insert(0, 0);
    visited.insert(internal, 1);

    for idx in (0..v.len()).rev() {
        let x = v[idx];

        let mut acc = 0;
        for i in 1..=3 {
            let plus = visited.get(&(x + i));
            if let Some(plus) = plus {
                acc += plus;
            }
        }

        // println!("x={} -> {}", x, acc);
        visited.insert(x, acc);
    }
    let elapsed = p2.elapsed();
    println!("Answer 2: {} took {:?}", visited[&0], elapsed);
}

#[inline]
fn find_differences(v: &Vec<usize>) -> (usize, usize) {
    let mut one_dif = 0usize;
    let mut three_dif = 1usize;

    let mut prev = 0;
    for x in v {
        match x - prev {
            1 => one_dif += 1,
            3 => three_dif += 1,
            d => panic!("unexpected diff {} ({} - {})", d, x, prev)
        }

        prev = *x;
    }

    (one_dif, three_dif)
}