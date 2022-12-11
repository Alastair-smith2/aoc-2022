use nom::{
    bytes::{complete::take_while1, streaming::tag},
    sequence::separated_pair,
    IResult,
};

#[derive(Debug)]
pub enum Direction {
    Up(usize),
    Left(usize),
    Right(usize),
    Down(usize),
}

pub fn parse_movements(input: &str) -> IResult<&str, Direction> {
    let (input, (movement, distance)) = separated_pair(
        take_while1(|c: char| "RUDL".contains(c)),
        tag(" "),
        nom::character::complete::u64,
    )(input)?;
    let direction = match movement {
        "D" => Direction::Down(distance as usize),
        "U" => Direction::Up(distance as usize),
        "L" => Direction::Left(distance as usize),
        "R" => Direction::Right(distance as usize),
        _ => panic!("Invalid character found"),
    };
    Ok((input, direction))
}
