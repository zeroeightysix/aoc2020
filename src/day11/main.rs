use crate::Occupation::*;
use std::time::Instant;

fn main() {
    const INPUT: &str = include_str!("input");

    let mut seats: Vec<Vec<Occupation>> = INPUT.lines().map(|l| l.chars().map(|c| match c {
        '.' => Floor,
        'L' => Empty,
        _ => panic!("invalid input")
    }).collect()).collect();
    let mut prev_occupied = 0;

    {
        let mut seats = seats.clone();
        let now = Instant::now();
        let occupied = loop {
            let (new_seats, occupied) = simulate_round_1(seats);

            if prev_occupied - occupied == 0 {
                break occupied;
            }

            prev_occupied = occupied;
            seats = new_seats;
        };
        let elapsed = now.elapsed();
        println!("Answer 1: {}, took {:?}", occupied, elapsed);
    }

    {
        let now = Instant::now();
        let occupied = loop {
            // print_seats(&seats);
            // println!("\n\n");
            let (new_seats, occupied) = simulate_round_2(seats);

            if prev_occupied - occupied == 0 {
                break occupied;
            }

            prev_occupied = occupied;
            seats = new_seats;
        };
        let elapsed = now.elapsed();
        println!("Answer 2: {}, took {:?}", occupied, elapsed);
    }
}

fn simulate_round_1(seats: Vec<Vec<Occupation>>) -> (Vec<Vec<Occupation>>, i32) {
    const ADJACENT: [(isize, isize); 8] = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];
    let mut new_seats = seats.clone();
    let mut occupied_count = 0;

    let is_occupied = |x: usize, y: usize| {
        if let Some(row) = seats.get(y) {
            if let Some(seat) = row.get(x) {
                return *seat == Occupation::Seated;
            }
        }
        return false;
    };

    let width = seats.first().unwrap().len();
    for y in 0..seats.len() as isize {
        for x in 0..width as isize {
            let seat = &seats[y as usize][x as usize];
            if let Empty = seat {
                if ADJACENT.iter().all(|(dx, dy)| !is_occupied((x + dx) as usize, (y + dy) as usize)) {
                    new_seats[y as usize][x as usize] = Occupation::Seated;
                    occupied_count += 1;
                }
            } else if let Seated = seat {
                if ADJACENT.iter().filter(|(dx, dy)| is_occupied((x + dx) as usize, (y + dy) as usize)).count() >= 4 {
                    new_seats[y as usize][x as usize] = Occupation::Empty;
                } else {
                    occupied_count += 1;
                }
            }
        }
    }

    (new_seats, occupied_count)
}

fn simulate_round_2(seats: Vec<Vec<Occupation>>) -> (Vec<Vec<Occupation>>, i32) {
    const ADJACENT: [(isize, isize); 8] = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];
    let mut new_seats = seats.clone();
    let mut occupied_count = 0;
    let width = seats.first().unwrap().len();

    let get_seat = |x: usize, y: usize| {
        if let Some(row) = seats.get(y) {
            if let Some(seat) = row.get(x) {
                return Some(seat)
            }
        }
        return None;
    };

    let first_seen = |mut x: isize, mut y: isize, (dx, dy): (isize, isize)| {
        loop {
            x += dx;
            y += dy;
            if x < 0 || y < 0 || x >= width as isize || y >= seats.len() as isize {
                break None
            }

            let seat = get_seat(x as usize, y as usize);

            if let Some(&Floor) = seat.map(|s| s) {
                continue
            }

            break seat
        }
    };

    for y in 0..seats.len() as isize {
        for x in 0..width as isize {
            let seat = &seats[y as usize][x as usize];
            if let Empty = seat {
                if ADJACENT.iter().all(|delta| {
                    let first_seen = first_seen(x, y, *delta);
                    first_seen != Some(&Seated)
                }) {
                    new_seats[y as usize][x as usize] = Occupation::Seated;
                    occupied_count += 1;
                }
            } else if let Seated = seat {
                if ADJACENT.iter().filter(|delta| {
                    first_seen(x, y, **delta) == Some(&Seated)
                }).count() >= 5 {
                    new_seats[y as usize][x as usize] = Occupation::Empty;
                } else {
                    occupied_count += 1;
                }
            }
        }
    }

    (new_seats, occupied_count)
}

#[derive(Debug, Eq, PartialEq, Clone)]
enum Occupation {
    Floor,
    Empty,
    Seated,
}