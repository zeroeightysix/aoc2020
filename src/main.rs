use scan_fmt::scan_fmt;

fn count_matches(input: &str) -> usize {
    input.lines()
        .filter(|l| {
            let (first, second, char, pw) = scan_fmt!(l, "{}-{} {}: {}", usize, usize, char, String).unwrap();
            let pw = pw.as_bytes();
            let char = char as u8;
            (pw[first - 1] == char) ^ (pw[second - 1] == char)
        })
        .count()
}

fn main() {
    println!("{}", count_matches(include_str!("input")));
}
