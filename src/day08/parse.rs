use crate::parsers::number;

use super::Instruction;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, line_ending, space1},
    combinator::value,
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

pub fn signed_number(input: &str) -> IResult<&str, isize> {
    let (input, sign) = alt((value(-1, char('-')), value(1, char('+'))))(input)?;
    let (input, abs) = number::<isize>(input)?;
    Ok((input, sign * abs))
}

pub fn nop(input: &str) -> IResult<&str, Instruction> {
    let (input, (_, value)) = separated_pair(tag("nop"), space1, signed_number)(input)?;
    Ok((input, Instruction::Nop(value)))
}
pub fn jmp(input: &str) -> IResult<&str, Instruction> {
    let (input, (_, value)) = separated_pair(tag("jmp"), space1, signed_number)(input)?;
    Ok((input, Instruction::Jmp(value)))
}
pub fn acc(input: &str) -> IResult<&str, Instruction> {
    let (input, (_, value)) = separated_pair(tag("acc"), space1, signed_number)(input)?;
    Ok((input, Instruction::Acc(value)))
}
pub fn instruction(input: &str) -> IResult<&str, Instruction> {
    alt((nop, jmp, acc))(input)
}

pub fn program(input: &str) -> IResult<&str, Vec<Instruction>> {
    separated_list1(line_ending, instruction)(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_program() {
        use Instruction::*;

        let input = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";
        let (input, instructions) = program(input).unwrap();
        assert_eq!(input, "");
        assert_eq!(
            instructions,
            vec![
                Nop(0),
                Acc(1),
                Jmp(4),
                Acc(3),
                Jmp(-3),
                Acc(-99),
                Acc(1),
                Jmp(-4),
                Acc(6),
            ]
        )
    }
}
