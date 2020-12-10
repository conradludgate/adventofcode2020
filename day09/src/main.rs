mod parse;

fn find_sum_pair(numbers: &[usize], sum: usize) -> Option<(usize, usize)> {
    for &number in numbers {
        if number > sum {
            continue;
        }
        let inverse = sum - number;
        if numbers.contains(&inverse) {
            return Some((number, inverse));
        }
    }
    None
}

fn find_invalid(numbers: &[usize], length: usize) -> Option<usize> {
    for i in length..numbers.len() {
        let sum = find_sum_pair(&numbers[(i - length)..i], numbers[i]);
        if sum.is_none() {
            return Some(numbers[i]);
        }
    }
    None
}

fn find_sum_contiguous(numbers: &[usize], sum: usize) -> &[usize] {
    let mut i = 0;
    let mut j = 1;
    loop {
        let s: usize = numbers[i..j].iter().sum();
        if s == sum {
            return &numbers[i..j];
        } else if s > sum {
            i += 1;
        } else {
            j += 1;
        }
    }
}

fn main() {
    let input = parse::read_file();
    let (_, numbers) = parse::numbers(&input).unwrap();
    let invalid = find_invalid(&numbers, 25).unwrap();
    println!("first invalid number: {}", invalid);

    let contiguous_sum = find_sum_contiguous(&numbers, invalid);
    println!("contiguous sum: {:?}", contiguous_sum);
    let max = contiguous_sum.iter().max().unwrap();
    let min = contiguous_sum.iter().min().unwrap();

    println!("min + max: {}", min + max);
}

#[test]
fn find_sum_pair_test() {
    let output = find_sum_pair(&vec![1721, 979, 366, 299, 675, 1456], 2020);
    assert_eq!(output, Some((1721, 299)));
}

#[test]
fn test_invalid() {
    let numbers = vec![
        35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309, 576,
    ];
    let invalid = find_invalid(&numbers, 5);
    assert_eq!(invalid, Some(127))
}

#[test]
fn test_sum_contiguous() {
    let numbers = vec![
        35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309, 576,
    ];
    let sum = find_sum_contiguous(&numbers, 127);
    assert_eq!(sum, vec![15, 25, 47, 40])
}
