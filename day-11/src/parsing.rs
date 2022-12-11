use crate::monkey::*;
use nom::{
    branch::alt,
    bytes::{complete::take_until, streaming::tag},
    character::{
        complete::{self},
        streaming::space1,
    },
    combinator::map,
    IResult,
};

fn trim_white_space(input: &str) -> IResult<&str, &str> {
    space1(input)
}

fn parse_monkey_false_destination(input: &str) -> IResult<&str, u32> {
    let (input, _) = trim_white_space(input)?;
    let (input, _) = tag("If false: throw to monkey ")(input)?;
    complete::u32(input)
}

fn parse_monkey_true_destination(input: &str) -> IResult<&str, u32> {
    let (input, _) = trim_white_space(input)?;
    let (input, _) = tag("If true: throw to monkey ")(input)?;
    complete::u32(input)
}

fn parse_old_operation_type(input: &str) -> IResult<&str, OperationType> {
    map(complete::u64, |v| OperationType::Number(v))(input)
}

fn parse_number_operation_type(input: &str) -> IResult<&str, OperationType> {
    map(tag("old"), |_| OperationType::Old)(input)
}

fn parse_operation_type(input: &str) -> IResult<&str, OperationType> {
    alt((parse_number_operation_type, parse_old_operation_type))(input)
}

fn parse_multiplication_operation(input: &str) -> IResult<&str, Operation> {
    let (input, _) = tag("* ")(input)?;
    let (input, operation_type) = parse_operation_type(input)?;
    let operation = Operation::Multiplication(operation_type);
    Ok((input, operation))
}

fn parse_addition_operation(input: &str) -> IResult<&str, Operation> {
    let (input, _) = tag("+ ")(input)?;
    let (input, operation_type) = parse_operation_type(input)?;
    let operation = Operation::Addition(operation_type);
    Ok((input, operation))
}

fn parse_monkey_operation(input: &str) -> IResult<&str, Operation> {
    let (input, _) = trim_white_space(input)?;
    let (input, _) = tag("Operation: new = old ")(input)?;
    alt((parse_multiplication_operation, parse_addition_operation))(input)
}

fn parse_monkey_test_factor(input: &str) -> IResult<&str, u64> {
    let (input, _) = trim_white_space(input)?;
    let (input, _) = tag("Test: divisible by ")(input)?;
    complete::u64(input)
}

fn parse_monkey_starting_items(input: &str) -> IResult<&str, Vec<u64>> {
    let (input, _) = trim_white_space(input)?;
    let (input, _) = tag("Starting items: ")(input)?;
    let numbers = input
        .split(", ")
        .map(|n| n.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    Ok((input, numbers))
}

fn parse_monkey_identifier(input: &str) -> IResult<&str, u32> {
    let (input, _) = tag("Monkey ")(input)?;
    let (_, val) = take_until(":")(input)?;
    let (input, id) = complete::u32(val)?;
    Ok((input, id))
}

pub fn parse_monkey(input: &str) -> IResult<&str, Monkey> {
    let monkey_input = input.split("\n").collect::<Vec<_>>();
    let (_, id) = parse_monkey_identifier(monkey_input[0])?;
    let (_, items) = parse_monkey_starting_items(monkey_input[1])?;
    let (_, operation) = parse_monkey_operation(monkey_input[2])?;
    let (_, test_factor) = parse_monkey_test_factor(monkey_input[3])?;
    let (_, true_destination) = parse_monkey_true_destination(monkey_input[4])?;
    let (_, false_destination) = parse_monkey_false_destination(monkey_input[5])?;
    let monkey = Monkey {
        id,
        items,
        operation,
        test_factor,
        true_destination,
        false_destination,
        inspection_count: 0,
    };

    Ok((input, monkey))
}
