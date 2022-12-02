mod day01;
mod day02;

fn main() {
    println!("----------------------------------");
    println!("Day 01");
    day01::run();
    println!("----------------------------------");
    println!("Day 02");
    day02::run();
    println!("----------------------------------");

    println!(
        "{}",
        include_bytes!("../inputs/day02.txt")
            .split(|b| *b == b'\n')
            .map(|l| ((l[0] - b'A') as i16, (l[2] - b'X') as i16,))
            .map(|(a, b)| 1 + b + 3 * ((1 + b - a).rem_euclid(3)))
            .sum::<i16>(),
    );
}
