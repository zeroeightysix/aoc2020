use scan_fmt::scan_fmt;

const INPUT: &str = include_str!("input");

fn count_matches() -> usize {
    INPUT.lines()
        .filter(|l| {
            let (min, max, char, pw) = scan_fmt!(l, "{}-{} {}: {}", usize, usize, char, String).unwrap();
            (min..=max).contains(&pw.chars().filter(|c| c == &char).count())
        })
        .count()
}

fn main() {
    println!("{}", count_matches());
}
