use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self},
    combinator::map,
    sequence::separated_pair,
    *,
};

#[derive(Debug)]
pub enum Command {
    Noop,
    AddX(i64),
}

fn parse_addx(input: &str) -> IResult<&str, Command> {
    map(separated_pair(tag("addx"), tag(" "), complete::i64), |v| {
        Command::AddX(v.1)
    })(input)
}

fn parse_noop(input: &str) -> IResult<&str, Command> {
    let (input, _) = tag("noop")(input)?;
    Ok((input, Command::Noop))
}

fn parse_command(input: &str) -> IResult<&str, Command> {
    alt((parse_addx, parse_noop))(input)
}

pub fn get_input(input: &str) -> Vec<Command> {
    input
        .lines()
        .map(|l| parse_command(l).unwrap().1)
        .collect::<Vec<_>>()
}
