use crate::parsers::{lines, number, all};
use crate::Challenge;

pub struct Day10 {
    runs: Vec<usize>,
}

impl Challenge for Day10 {
    fn name() -> &'static str {
        "day10"
    }
    fn new(input: String) -> Self {
        Day10 {
            runs: runs(all(lines(number)(&input))),
        }
    }
    fn part_one(&self) -> usize {
        let (ones, threes) = gaps(&self.runs);
        ones * threes
    }
    fn part_two(&self) -> usize {
        arrangements(&self.runs)
    }
}

fn runs(mut numbers: Vec<usize>) -> Vec<usize> {
    numbers.sort_unstable();

    let mut runs = vec![];

    let mut run = 1;
    for i in 1..numbers.len() {
        let diff = numbers[i] - numbers[i - 1];
        if diff == 1 {
            run += 1;
        } else if diff == 3 {
            runs.push(run + 1);
            run = 0;
        } else {
            panic!(format!("diff != 1 or 3: {}", diff))
        }
    }
    runs.push(run + 1);

    runs
}

fn gaps(runs: &Vec<usize>) -> (usize, usize) {
    let threes = runs.len();
    let ones = runs.iter().sum::<usize>() - threes;
    (ones, threes)
}

fn arrangements(runs: &Vec<usize>) -> usize {
    let map = vec![0, 1, 1, 2, 4, 7]; // hard coded permutation map
    runs.into_iter()
        .map(|&run| map[run])
        .fold(1, |acc, x| acc * x)
}

#[test]
fn test_runs_1() {
    let numbers = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
    let runs = runs(numbers);
    assert_eq!(runs, vec![2, 4, 3, 2, 1]);
}

#[test]
fn test_runs_2() {
    let numbers = vec![
        28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35, 8,
        17, 7, 9, 4, 2, 34, 10, 3,
    ];
    let runs = runs(numbers);
    assert_eq!(runs, vec![5, 5, 1, 4, 3, 1, 5, 2, 1, 5]);
}

#[test]
fn test_gaps_1() {
    let numbers = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
    let (ones, threes) = gaps(&runs(numbers));
    assert_eq!(ones, 7);
    assert_eq!(threes, 5)
}

#[test]
fn test_gaps_2() {
    let numbers = vec![
        28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35, 8,
        17, 7, 9, 4, 2, 34, 10, 3,
    ];
    let (ones, threes) = gaps(&runs(numbers));
    assert_eq!(ones, 22);
    assert_eq!(threes, 10)
}

#[test]
fn test_arrangements_1() {
    let numbers = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
    let arrangements = arrangements(&runs(numbers));
    assert_eq!(arrangements, 8);
}

#[test]
fn test_arrangements_2() {
    let numbers = vec![
        28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35, 8,
        17, 7, 9, 4, 2, 34, 10, 3,
    ];
    let arrangements = arrangements(&runs(numbers));
    assert_eq!(arrangements, 19208);
}
