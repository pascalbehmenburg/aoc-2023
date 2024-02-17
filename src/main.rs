#![feature(iter_array_chunks)]
mod day01;
mod day02;
fn main() {
    let day_one_input = day01::parse(include_str!("../resources/one.txt"));
    println!("D01 Part 1: {}", day01::part1(&day_one_input));
    println!("D01 Part 2: {}", day01::part2(&day_one_input));

    let day_two_input = day02::parse(include_str!("../resources/two.txt"));
    println!("D02 Part 1: {}", day02::part1(&day_two_input));
    println!("D02 Part 2: {}", day02::part2(&day_two_input));

    //day_two()?;
    //day_three()
}
