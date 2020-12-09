mod parse;

// from day01
fn find_sum<T>(numbers: &[T], sum: T, n: usize) -> Option<Vec<T>>
where
    T: std::ops::Add<Output = T> + std::ops::Sub<Output = T> + PartialEq + PartialOrd + Copy,
{
    match n {
        0 => Some(vec![]),
        1 => {
            if numbers.contains(&sum) {
                Some(vec![sum])
            } else {
                None
            }
        }
        _ => {
            for i in 0..numbers.len() {
                if numbers[i] < sum {
                    let attempt = find_sum(&numbers[i + 1..], sum - numbers[i], n - 1);
                    if let Some(mut v) = attempt {
                        v.push(numbers[i]);
                        return Some(v);
                    }
                }
            }
            None
        }
    }
}

#[test]
fn find_sum_pair_test() {
    let output = find_sum(&vec![1721, 979, 366, 299, 675, 1456], 2020, 2);
    assert_eq!(output, Some(vec![299, 1721]));
}

fn find_invalid(numbers: &[usize], length: usize) -> Option<usize> {
    for i in length..numbers.len() {
        let sum = find_sum(&numbers[(i - length)..i], numbers[i], 2);
        if sum.is_none() {
            return Some(numbers[i]);
        }
    }
    None
}

#[test]
fn test_invalid() {
    let numbers = vec![
        35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309, 576,
    ];
    let invalid = find_invalid(&numbers, 5);
    assert_eq!(invalid, Some(127))
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

#[test]
fn test_sum_contiguous() {
    let numbers = vec![
        35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309, 576,
    ];
    let sum = find_sum_contiguous(&numbers, 127);
    assert_eq!(sum, vec![15, 25, 47, 40])
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
