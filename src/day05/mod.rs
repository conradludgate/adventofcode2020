use nom::{
    branch::alt,
    character::complete::{char, line_ending},
    combinator::value,
    multi::{count, separated_list1},
    IResult,
};

use crate::Challenge;

pub struct Day05 {
    seat_ids: Vec<usize>,
}

impl Challenge for Day05 {
    fn name() -> &'static str {
        "day05"
    }
    fn new(input: String) -> Self {
        let (_, seats) = parse_seats(&input).unwrap();
        let seat_ids = seats.into_iter().map(to_seat_id).collect();
        Day05 { seat_ids }
    }
    fn part_one(&self) -> usize {
        self.seat_ids.iter().fold(0, |a, &id| a.max(id))
    }
    fn part_two(&self) -> usize {
        let max_id = self.seat_ids.iter().fold(0, |a, &id| a.max(id));
        let min_id = self.seat_ids.iter().fold(max_id, |a, &id| a.min(id));
        
        let mut seats: Vec<bool> = Vec::with_capacity(128 * 8);
        seats.resize(128 * 8, false);

        for &seat_id in self.seat_ids.iter() {
            seats[seat_id] = true;
        }

        for (id, &seat) in seats.iter().enumerate() {
            if !seat && (min_id..=max_id).contains(&id) {
                return id;
            }
        }
        panic!("no seat found");
    }
}

fn parse_fb(input: &str) -> IResult<&str, usize> {
    alt((value(0, char('F')), value(1, char('B'))))(input)
}
fn parse_lr(input: &str) -> IResult<&str, usize> {
    alt((value(0, char('L')), value(1, char('R'))))(input)
}

fn parse_row(input: &str) -> IResult<&str, usize> {
    let (input, bits) = count(parse_fb, 7)(input)?;
    let row = bits.into_iter().fold(0, |a, b| a << 1 | b);
    Ok((input, row))
}

fn parse_col(input: &str) -> IResult<&str, usize> {
    let (input, bits) = count(parse_lr, 3)(input)?;
    let col = bits.into_iter().fold(0, |a, b| a << 1 | b);
    Ok((input, col))
}

fn parse_seat(input: &str) -> IResult<&str, (usize, usize)> {
    let (input, row) = parse_row(input)?;
    let (input, col) = parse_col(input)?;
    Ok((input, (row, col)))
}

fn parse_seats(input: &str) -> IResult<&str, Vec<(usize, usize)>> {
    separated_list1(line_ending, parse_seat)(input)
}

fn to_seat_id((row, col): (usize, usize)) -> usize {
    row * 8 + col
}

#[test]
fn test_seat_id() {
    assert_eq!(to_seat_id((70, 7)), 567);
    assert_eq!(to_seat_id((14, 7)), 119);
    assert_eq!(to_seat_id((102, 4)), 820);
}

#[test]
fn test_parse_row() {
    let (input, row) = parse_row("BFFFBBF").unwrap();
    assert_eq!(input.len(), 0);
    assert_eq!(row, 70);

    let (input, row) = parse_row("FFFBBBF").unwrap();
    assert_eq!(input.len(), 0);
    assert_eq!(row, 14);

    let (input, row) = parse_row("BBFFBBF").unwrap();
    assert_eq!(input.len(), 0);
    assert_eq!(row, 102);
}

#[test]
fn test_parse_col() {
    let (input, col) = parse_col("RRR").unwrap();
    assert_eq!(input.len(), 0);
    assert_eq!(col, 7);

    let (input, col) = parse_col("RLL").unwrap();
    assert_eq!(input.len(), 0);
    assert_eq!(col, 4);
}

#[test]
fn test_parse_seat() {
    let (input, seat) = parse_seat("BFFFBBFRRR").unwrap();
    assert_eq!(input.len(), 0);
    assert_eq!(seat, (70, 7));

    let (input, seat) = parse_seat("FFFBBBFRRR").unwrap();
    assert_eq!(input.len(), 0);
    assert_eq!(seat, (14, 7));

    let (input, seat) = parse_seat("BBFFBBFRLL").unwrap();
    assert_eq!(input.len(), 0);
    assert_eq!(seat, (102, 4));
}

#[test]
fn test_parse_seats() {
    let (input, seats) = parse_seats("BFFFBBFRRR\nFFFBBBFRRR\nBBFFBBFRLL").unwrap();
    assert_eq!(input.len(), 0);
    assert_eq!(seats, vec![(70, 7), (14, 7), (102, 4),]);
}
