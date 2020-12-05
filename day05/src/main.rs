use nom::{
    alt, character::complete::line_ending, complete, multi::count, multi::many1, named, tag,
    IResult,
};

named!(parse_fb<&str, usize>, alt!(
    complete!(tag!("F")) => { |_| 0 } |
    complete!(tag!("B")) => { |_| 1 }
));
named!(parse_lr<&str, usize>, alt!(
    complete!(tag!("L")) => { |_| 0 } |
    complete!(tag!("R")) => { |_| 1 }
));

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
    let (input, _) = line_ending(input)?;
    Ok((input, (row, col)))
}

fn parse_seats(input: &str) -> IResult<&str, Vec<(usize, usize)>> {
    many1(parse_seat)(input)
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

fn to_seat_id((row, col): (usize, usize)) -> usize {
    row * 8 + col
}

fn main() {
    let input = read_file();
    let (_, seats) = parse_seats(&input).expect("could not parse file");
    let seat_ids: Vec<usize> = seats.into_iter().map(to_seat_id).collect();
    let max_ids = seat_ids.iter().fold(0, |a, &id| a.max(id));
    println!("max id: {}", max_ids);

    let mut seats: Vec<bool> = Vec::with_capacity(128 * 8);
    seats.resize(128*8, false);

    for seat_id in seat_ids {
        seats[seat_id] = true;
    }

    for (id, &seat) in seats.iter().enumerate() {
        if !seat {
            println!("empty id: {}", id);
        }
    }
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
    let (input, seat) = parse_seat("BFFFBBFRRR\n").unwrap();
    assert_eq!(input.len(), 0);
    assert_eq!(seat, (70, 7));

    let (input, seat) = parse_seat("FFFBBBFRRR\n").unwrap();
    assert_eq!(input.len(), 0);
    assert_eq!(seat, (14, 7));

    let (input, seat) = parse_seat("BBFFBBFRLL\n").unwrap();
    assert_eq!(input.len(), 0);
    assert_eq!(seat, (102, 4));
}

#[test]
fn test_parse_seats() {
    let (input, seats) = parse_seats("BFFFBBFRRR\nFFFBBBFRRR\nBBFFBBFRLL\n").unwrap();
    assert_eq!(input.len(), 0);
    assert_eq!(seats, vec![(70, 7), (14, 7), (102, 4),]);
}
