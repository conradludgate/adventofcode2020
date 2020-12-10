mod parse;

fn gaps(numbers: &[usize]) -> (usize, usize) {

    let mut ones = 0;
    let mut threes = 1;

    if numbers[0] == 1 {
        ones += 1;
    } else if numbers[0] == 3 {
        threes += 1;
    }

    for i in 1..numbers.len() {
        let diff = numbers[i] - numbers[i - 1];
        if diff == 1 {
            ones += 1;
        }
        if diff == 3 {
            threes += 1;
        }
    }

    (ones, threes)
}

#[test]
fn test_gaps_1() {
    let mut numbers = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
    numbers.sort_unstable();
    let (ones, threes) = gaps(&numbers);
    assert_eq!(ones, 7);
    assert_eq!(threes, 5)
}

#[test]
fn test_gaps_2() {
    let mut numbers = vec![
        28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35, 8,
        17, 7, 9, 4, 2, 34, 10, 3,
    ];
    numbers.sort_unstable();
    let (ones, threes) = gaps(&numbers);
    assert_eq!(ones, 22);
    assert_eq!(threes, 10)
}

fn arrangements(numbers: &[usize]) -> usize {
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

    let map = vec![0, 1, 1, 2, 4, 7];
    runs.into_iter().map(|run| map[run]).fold(1, |acc, x| acc * x)
}

#[test]
fn test_arrangements_1() {
    let mut numbers = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
    numbers.sort_unstable();

    let arrangements = arrangements(&numbers);
    assert_eq!(arrangements, 8);
}

#[test]
fn test_arrangements_2() {
    let mut numbers = vec![
        28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35, 8,
        17, 7, 9, 4, 2, 34, 10, 3,
    ];
    numbers.sort_unstable();

    let arrangements = arrangements(&numbers);
    assert_eq!(arrangements, 19208);
}

fn main() {
    let input = parse::read_file();
    let (_, mut numbers) = parse::numbers(&input).unwrap();
    numbers.sort_unstable();

    let (ones, threes) = gaps(&numbers);
    println!(
        "ones: {}, three: {}, answer: {}",
        ones,
        threes,
        ones * threes
    );

    let arrangements = arrangements(&numbers);
    println!("arrangements: {}", arrangements)
}
