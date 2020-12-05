const INPUT: &str = include_str!("input");

fn main() {
    let input: Vec<usize> = INPUT.split('\n').map(|x| x.parse().expect("not a number")).collect();

    for x in &input {
        for y in &input {
            for z in &input {
                if x + y + z == 2020 {
                    println!("{} + {} + {} = 2020", x, y, z);
                    println!("{} x {} x {} = {}", x, y, z, x * y * z);
                }
            }
        }
    }

    println!("{:?}", input);
}
