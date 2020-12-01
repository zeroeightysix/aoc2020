const INPUT: &str = include_str!("input");

fn main() {
    let input: Vec<usize> = INPUT.split('\n').map(|x| x.parse().expect("not a number")).collect();

    for x in &input {
        for y in &input {
            if x + y == 2020 {
                println!("{} + {} = 2020", x, y);
                println!("{} x {} = {}", x, y, x * y);
            }
        }
    }

    println!("{:?}", input);
}
