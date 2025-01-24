use std::thread::ScopedJoinHandle;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{self, anychar};
use nom::combinator::value;
use nom::multi::{many1, many_till};
use nom::sequence::{delimited, separated_pair};
use nom::IResult;
use nom::Parser;

fn main() {
    let input = include_str!("./input.txt");
    let output = part2(input);
    dbg!(output);
}

#[derive(Clone, Debug)]
enum Instruction {
    Mul(u32, u32),
    Do,
    Dont,
}

#[derive(PartialEq, Eq)]
enum ShouldProcess {
    Do,
    Dont,
}

fn mul(input: &str) -> IResult<&str, Instruction> {
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

fn instruction(input: &str) -> IResult<&str, Instruction> {
    alt((
        value(Instruction::Dont, tag("don't()")),
        value(Instruction::Do, tag("do()")),
        mul,
    ))(input)
}

fn parse(input: &str) -> IResult<&str, Vec<Instruction>> {
    return many1(
        many_till(anychar, instruction)
            .map(|(_discard, ins)| ins),
    )(input);
}

fn part2(input: &str) -> u32 {
    let (_, instruction) = parse(input).map_err(|e| dbg!(e)).unwrap();
    let (_, result) = instruction.iter().fold(
        (ShouldProcess::Do, 0), |(process, acc), ins| {
            match ins {
                Instruction::Mul(a, b) => {
                    if process == ShouldProcess::Do {
                        (process, acc + a * b)
                    } else {
                        (process, acc)
                    }
                }
                Instruction::Do => (ShouldProcess::Do, acc),
                Instruction::Dont => (ShouldProcess::Dont, acc),
            }
        }
    );

    return result;
}

#[cfg(test)]
mod tests {
    use crate::part2;

    #[test]
    fn part2_test() {
        let result = part2("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))");
        assert_eq!(result, 48);
    }
}