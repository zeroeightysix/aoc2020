fn main() {
    const INPUT: &str = include_str!("input");
    let lines: Vec<Vec<bool>> = INPUT.lines()
        .map(|l| l.chars().map(|c| c == '#').collect::<Vec<bool>>())
        .collect();

    println!("Answer 1: {}", trees_with_slope(&lines, (3, 1)));

    let p: usize = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)].iter().map(|slope| {
        trees_with_slope(&lines, *slope)
    }).product();

    println!("Answer 2: {}", p);
}

fn trees_with_slope(lines: &Vec<Vec<bool>>, slope: (usize, usize)) -> usize {
    let width = lines.iter().nth(0).unwrap().len();
    let (dx, dy) = slope;
    let (mut x, mut y) = (0, 0);
    let mut trees = 0;
    while y < lines.len() {
        trees += lines[y][x] as usize;
        x = (x + dx) % width;
        y += dy;
    }

    trees
}