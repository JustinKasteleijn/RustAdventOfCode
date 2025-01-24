use nom::bytes::complete::tag;
use nom::character::complete::{self, anychar};
use nom::multi::{many1, many_till};
use nom::sequence::{delimited, separated_pair};
use nom::IResult;
use nom::Parser;

fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

enum Instruction {
    Mul(u32, u32)
}

fn instruction(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag("mul")(input)?;
    let (input, pair) = delimited(
        tag("("),
        separated_pair(
            complete::u32,
            tag(","),
            complete::u32,
        ),
        tag(")"),
    )(input)?;
    return Ok((input, Instruction::Mul(pair.0, pair.1)));
}
fn parse(input: &str) -> IResult<&str, Vec<Instruction>> {
    return many1(
        many_till(anychar, instruction)
            .map(|(_discard, ins)| ins),
    )(input);
}

fn part1(input: &str) -> u32 {
    let (_, input) = parse(input).map_err(|e| dbg!(e)).unwrap();
    return input
        .iter()
        .map(|ins| match ins {
            Instruction::Mul(a, b) => a * b
        }).sum::<u32>();
}

#[cfg(test)]
mod tests {
    use crate::part1;

    #[test]
    fn part1_test() {
        let result = part1("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))");
        assert_eq!(result, 161);
    }
}