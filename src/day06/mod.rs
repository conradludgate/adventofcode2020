use nom::{
    branch::alt,
    character::complete::{char, line_ending},
    combinator::value,
    multi::{count, many1, separated_list1},
    IResult,
};

use crate::Challenge;

pub struct Day06 {
    group_answers: Vec<Vec<Vec<Answer>>>,
}

impl Challenge for Day06 {
    fn name() -> &'static str {
        "day06"
    }
    fn new(input: String) -> Self {
        Day06 {
            group_answers: parse_all_group_answers(&input).unwrap().1,
        }
    }
    fn part_one(&self) -> usize {
        count_all_group_answers(&self.group_answers) as usize
    }
    fn part_two(&self) -> usize {
        count_all_group_answers2(&self.group_answers) as usize
    }
}

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

fn parse_answer(input: &str) -> IResult<&str, Answer> {
    alt((
        alt((
            value(Answer::A, char('a')),
            value(Answer::B, char('b')),
            value(Answer::C, char('c')),
            value(Answer::D, char('d')),
            value(Answer::E, char('e')),
            value(Answer::F, char('f')),
            value(Answer::G, char('g')),
            value(Answer::H, char('h')),
            value(Answer::I, char('i')),
            value(Answer::J, char('j')),
            value(Answer::K, char('k')),
            value(Answer::L, char('l')),
            value(Answer::M, char('m')),
        )),
        alt((
            value(Answer::N, char('n')),
            value(Answer::O, char('o')),
            value(Answer::P, char('p')),
            value(Answer::Q, char('q')),
            value(Answer::R, char('r')),
            value(Answer::S, char('s')),
            value(Answer::T, char('t')),
            value(Answer::U, char('u')),
            value(Answer::V, char('v')),
            value(Answer::W, char('w')),
            value(Answer::X, char('x')),
            value(Answer::Y, char('y')),
            value(Answer::Z, char('z')),
        )),
    ))(input)
}

fn parse_answers(input: &str) -> IResult<&str, Vec<Answer>> {
    many1(parse_answer)(input)
}

fn parse_group_answers(input: &str) -> IResult<&str, Vec<Vec<Answer>>> {
    separated_list1(line_ending, parse_answers)(input)
}

fn parse_all_group_answers(input: &str) -> IResult<&str, Vec<Vec<Vec<Answer>>>> {
    separated_list1(count(line_ending, 2), parse_group_answers)(input)
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
