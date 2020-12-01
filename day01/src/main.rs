fn parse_input() -> Vec<usize> {
    use std::fs::File;
    use std::io::prelude::*;
    use std::io::BufReader;
    use std::str::FromStr;

    let file = File::open("input.txt").expect("could not open file");
    let mut buf_reader = BufReader::new(file);

    let mut numbers: Vec<usize> = vec![];
    loop {
        let mut line = String::new();
        let len = buf_reader
            .read_line(&mut line)
            .expect("could not read line");
        if len == 0 {
            break;
        }
        let number = usize::from_str(line.trim_end()).expect(&format!("invalid input, {:?}", line));
        numbers.push(number);
    }

    numbers
}

fn find_sum_pair(numbers: &Vec<usize>) -> Option<(usize, usize)> {
    use std::collections::HashSet;
    let mut number_set: HashSet<usize> = HashSet::new();
    for number in numbers {
        number_set.insert(*number);
    }
    for number in number_set.iter() {
        let inverse = 2020 - number;
        if number_set.contains(&inverse) {
            return Some((*number, inverse));
        }
    }
    None
}
#[test]
fn find_sum_pair_test() {
    let output = find_sum_pair(&vec![1721, 979, 366, 299, 675, 1456]);
    assert_eq!(output, Some((1721, 299)));
}

fn find_sum_trio(numbers: &Vec<usize>) -> Option<(usize, usize, usize)> {
    for i in 0..numbers.len() {
        for j in (i + 1)..numbers.len() {
            for k in (j + 1)..numbers.len() {
                if numbers[i] + numbers[j] + numbers[k] == 2020 {
                    return Some((numbers[i], numbers[j], numbers[k]));
                }
            }
        }
    }
    None
}

#[test]
fn find_sum_trio_test() {
    let output = find_sum_trio(&vec![1721, 979, 366, 299, 675, 1456]);
    assert_eq!(output, Some((979, 366, 675)));
}

fn main() {
    let numbers = parse_input();
    let (a, b) = find_sum_pair(&numbers).unwrap();
    println!("{} x {} = {}", a, b, a * b);

    let (a, b, c) = find_sum_trio(&numbers).unwrap();
    println!("{} x {} x {} = {}", a, b, c, a * b * c);
}
