use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::complete::space1,
    combinator::map,
    *,
};

// COMMANDS
#[derive(Debug)]
pub struct Move<'a>(pub &'a str);

#[derive(Debug)]
pub struct List;

#[derive(Debug)]
pub enum Command<'a> {
    Cd(Move<'a>),
    Ls(List),
}

fn parse_move<'a>(input: &'a str) -> IResult<&str, Move<'a>> {
    let (input, _) = tag("cd ")(input)?;
    // IResult((input, ))
    Ok((input, Move { 0: input }))
}

fn parse_list(input: &str) -> IResult<&str, List> {
    let (input, _) = tag("ls")(input)?;
    Ok((input, List))
}

fn parse_command(input: &str) -> IResult<&str, Command> {
    let (input, _) = tag("$ ")(input)?;
    alt((
        map(parse_move, |val| Command::Cd(val)),
        map(parse_list, |v| Command::Ls(v)),
    ))(input)
}

// ENTRIES

#[derive(Debug)]
pub enum Entry<'a> {
    Dir(Dir<'a>),
    File(File<'a>),
}

#[derive(Debug)]
pub struct Dir<'a>(pub &'a str);
#[derive(Debug)]
pub struct File<'a> {
    pub size: usize,
    pub name: &'a str,
}

fn parse_dir<'a>(input: &'a str) -> IResult<&str, Dir> {
    let (dir_name, unused) = tag("dir ")(input)?;
    Ok((unused, Dir { 0: dir_name }))
}

fn parse_file<'a>(input: &'a str) -> IResult<&str, File<'a>> {
    // pair?
    let (input, file_size) = take_while1(|c: char| c.is_digit(10))(input)?;
    let (input, _) = space1(input)?;
    let (input, file_path) =
        take_while1(|character: char| "abcdefghijklmnopqrstuvwxyz./".contains(character))(input)?;
    let number = file_size.parse::<u32>().unwrap();
    Ok((
        input,
        File {
            size: number as usize,
            name: file_path,
        },
    ))
}

fn parse_entry(input: &str) -> IResult<&str, Entry> {
    alt((
        map(parse_dir, |d| Entry::Dir(d)),
        map(parse_file, |f| Entry::File(f)),
    ))(input)
}

#[derive(Debug)]
pub enum Line<'a> {
    Command(Command<'a>),
    Entry(Entry<'a>),
}

pub fn parse_file_line(input: &str) -> Line {
    let (_, line) = alt((
        map(parse_command, |c| Line::Command(c)),
        map(parse_entry, |e| Line::Entry(e)),
    ))(input)
    .unwrap();
    line
}
