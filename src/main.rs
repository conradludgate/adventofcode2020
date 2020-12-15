use std::path::Path;
pub fn read_file<P: AsRef<Path>>(path: P) -> String {
    use std::fs::File;
    use std::io::prelude::*;

    let mut file = File::open(path).expect("could not open file");
    let mut input = String::new();
    file.read_to_string(&mut input)
        .expect("could not read file");
    input
}

pub(crate) trait Challenge: Sized {
    fn name() -> &'static str;
    fn new<'a>(input: String) -> Self;
    fn part_one(&self) -> usize;
    fn part_two(&self) -> usize;

    fn run() {
        let name = Self::name();
        let input = read_file(Path::new("src").join(Path::new(name).join("input.txt")));
        let challenge = Self::new(input);
        println!("\nRunning challenge {}", name);
        println!("\tAnswer to part one: {}", challenge.part_one());
        println!("\tAnswer to part two: {}\n", challenge.part_two());
    }
}

mod day01;
mod day02;
mod day03;
// mod day04;
// mod day05;
// mod day06;
// mod day07;
// mod day08;
// mod day09;
// mod day10;
// mod day11;
// mod day12;
// mod day13;

fn main() {
    day01::Day01::run();
    day02::Day02::run();
    day03::Day03::run();
}
