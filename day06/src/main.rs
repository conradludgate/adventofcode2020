use nom::{
    alt,
    character::complete::line_ending,
    complete,
    multi::many1,
    multi::{count, separated_list1},
    named, tag, IResult,
};

#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(u32)]
enum Answer {
    A = 0x00000001,
    B = 0x00000002,
    C = 0x00000004,
    D = 0x00000008,
    E = 0x00000010,
    F = 0x00000020,
    G = 0x00000040,
    H = 0x00000080,
    I = 0x00000100,
    J = 0x00000200,
    K = 0x00000400,
    L = 0x00000800,
    M = 0x00001000,
    N = 0x00002000,
    O = 0x00004000,
    P = 0x00008000,
    Q = 0x00010000,
    R = 0x00020000,
    S = 0x00040000,
    T = 0x00080000,
    U = 0x00100000,
    V = 0x00200000,
    W = 0x00400000,
    X = 0x00800000,
    Y = 0x01000000,
    Z = 0x02000000,
}
named!(parse_answer<&str, Answer>, alt!(
    complete!(tag!("a")) => { |_| Answer::A } |
    complete!(tag!("b")) => { |_| Answer::B } |
    complete!(tag!("c")) => { |_| Answer::C } |
    complete!(tag!("d")) => { |_| Answer::D } |
    complete!(tag!("e")) => { |_| Answer::E } |
    complete!(tag!("f")) => { |_| Answer::F } |
    complete!(tag!("g")) => { |_| Answer::G } |
    complete!(tag!("h")) => { |_| Answer::H } |
    complete!(tag!("i")) => { |_| Answer::I } |
    complete!(tag!("j")) => { |_| Answer::J } |
    complete!(tag!("k")) => { |_| Answer::K } |
    complete!(tag!("l")) => { |_| Answer::L } |
    complete!(tag!("m")) => { |_| Answer::M } |
    complete!(tag!("n")) => { |_| Answer::N } |
    complete!(tag!("o")) => { |_| Answer::O } |
    complete!(tag!("p")) => { |_| Answer::P } |
    complete!(tag!("q")) => { |_| Answer::Q } |
    complete!(tag!("r")) => { |_| Answer::R } |
    complete!(tag!("s")) => { |_| Answer::S } |
    complete!(tag!("t")) => { |_| Answer::T } |
    complete!(tag!("u")) => { |_| Answer::U } |
    complete!(tag!("v")) => { |_| Answer::V } |
    complete!(tag!("w")) => { |_| Answer::W } |
    complete!(tag!("x")) => { |_| Answer::X } |
    complete!(tag!("y")) => { |_| Answer::Y } |
    complete!(tag!("z")) => { |_| Answer::Z }
));

fn parse_answers(input: &str) -> IResult<&str, Vec<Answer>> {
    many1(parse_answer)(input)
}

fn parse_group_answers(input: &str) -> IResult<&str, Vec<Vec<Answer>>> {
    separated_list1(line_ending, parse_answers)(input)
}

fn parse_all_group_answers(input: &str) -> IResult<&str, Vec<Vec<Vec<Answer>>>> {
    separated_list1(count(line_ending, 2), parse_group_answers)(input)
}

fn read_file() -> String {
    use std::fs::File;
    use std::io::prelude::*;

    let mut file = File::open("input.txt").expect("could not open file");
    let mut input = String::new();
    file.read_to_string(&mut input)
        .expect("could not read file");
    input
}

fn count_group_answers(group_answers: &Vec<Vec<Answer>>) -> u32 {
    group_answers
        .iter()
        .fold(0, |a, answers| {
            answers.iter().fold(a, |a, &answer| a | (answer as u32))
        })
        .count_ones()
}

fn count_all_group_answers(all_group_answers: &Vec<Vec<Vec<Answer>>>) -> u32 {
    all_group_answers
        .iter()
        .fold(0, |a, group_answers| a + count_group_answers(group_answers))
}

fn count_group_answers2(group_answers: &Vec<Vec<Answer>>) -> u32 {
    group_answers
        .iter()
        .fold(0xffffffff, |a, answers| {
            a & answers.iter().fold(0, |a, &answer| a | (answer as u32))
        })
        .count_ones()
}

fn count_all_group_answers2(all_group_answers: &Vec<Vec<Vec<Answer>>>) -> u32 {
    all_group_answers.iter().fold(0, |a, group_answers| {
        a + count_group_answers2(group_answers)
    })
}

fn main() {
    let input = read_file();
    let (_, all_group_answers) = parse_all_group_answers(&input).expect("could not parse file");
    println!(
        "count sum: {:?}",
        count_all_group_answers(&all_group_answers)
    );
    println!(
        "count sum2: {:?}",
        count_all_group_answers2(&all_group_answers)
    );
}

#[test]
fn test_count_group_answers() {
    use Answer::*;
    let input = "abcx
abcy
abcz";
    let (input, group_answers) = parse_group_answers(input).unwrap();
    assert_eq!(input.len(), 0);
    assert_eq!(
        group_answers,
        vec![vec![A, B, C, X], vec![A, B, C, Y], vec![A, B, C, Z],]
    );

    let count = count_group_answers(&group_answers);
    assert_eq!(count, 6);
}

#[test]
fn test_count_all_group_answers() {
    let input = "abc

a
b
c

ab
ac

a
a
a
a

b";
    let (input, all_group_answers) = parse_all_group_answers(input).unwrap();
    assert_eq!(input.len(), 0);
    assert_eq!(all_group_answers.len(), 5);

    let count = count_all_group_answers(&all_group_answers);
    assert_eq!(count, 11);
}

#[test]
fn test_count_group_answers2() {
    use Answer::*;
    let input = "abcx
abcy
abcz";
    let (input, group_answers) = parse_group_answers(input).unwrap();
    assert_eq!(input.len(), 0);
    assert_eq!(
        group_answers,
        vec![vec![A, B, C, X], vec![A, B, C, Y], vec![A, B, C, Z],]
    );

    let count = count_group_answers2(&group_answers);
    assert_eq!(count, 3);
}

#[test]
fn test_count_all_group_answers2() {
    let input = "abc

a
b
c

ab
ac

a
a
a
a

b";
    let (input, all_group_answers) = parse_all_group_answers(input).unwrap();
    assert_eq!(input.len(), 0);
    assert_eq!(all_group_answers.len(), 5);

    let count = count_all_group_answers2(&all_group_answers);
    assert_eq!(count, 6);
}
