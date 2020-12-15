use super::Spot;
use nom::{branch::alt, character::complete::char, combinator::value, IResult};

pub fn spot(input: &str) -> IResult<&str, Spot> {
    alt((
        value(Spot::Floor, char('.')),
        value(Spot::Empty, char('L')),
        value(Spot::Fill, char('#')),
    ))(input)
}

#[test]
fn test_parse_rows() {
    use crate::parsers::grid;

    let input = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";
    let (input, grid) = grid(spot)(input).unwrap();
    assert_eq!(input, "");
    use Spot::*;
    assert_eq!(
        grid,
        vec![
            vec![Empty, Floor, Empty, Empty, Floor, Empty, Empty, Floor, Empty, Empty],
            vec![Empty, Empty, Empty, Empty, Empty, Empty, Empty, Floor, Empty, Empty],
            vec![Empty, Floor, Empty, Floor, Empty, Floor, Floor, Empty, Floor, Floor],
            vec![Empty, Empty, Empty, Empty, Floor, Empty, Empty, Floor, Empty, Empty],
            vec![Empty, Floor, Empty, Empty, Floor, Empty, Empty, Floor, Empty, Empty],
            vec![Empty, Floor, Empty, Empty, Empty, Empty, Empty, Floor, Empty, Empty],
            vec![Floor, Floor, Empty, Floor, Empty, Floor, Floor, Floor, Floor, Floor],
            vec![Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty],
            vec![Empty, Floor, Empty, Empty, Empty, Empty, Empty, Empty, Floor, Empty],
            vec![Empty, Floor, Empty, Empty, Empty, Empty, Empty, Floor, Empty, Empty],
        ]
    )
}
