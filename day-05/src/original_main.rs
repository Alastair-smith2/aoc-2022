use regex::Regex;
use std::error::Error;

extern crate nom;

fn main() -> Result<(), Box<dyn Error>> {
    let input = include_str!("../input.txt");
    // let part_one_solution = part_one_solution(input)?;
    // println!("Part one {part_one_solution}");
    let part_two_solution = part_two_solution(input)?;
    println!("Part two {part_two_solution}");
    Ok(())
}

#[derive(Debug)]
struct Move {
    movement_distance: u32,
    from: u32,
    destination: u32,
}

#[derive(Debug, Clone)]
struct Stack<'a> {
    items: Vec<&'a str>,
}

impl<'a> Stack<'a> {
    fn get_top_item(&self) -> &'a str {
        self.items.first().unwrap()
    }

    fn add_item(&mut self, item: &'a str) {
        self.items.push(item);
    }
}

#[derive(Debug)]
struct Stacks<'a> {
    rows: Vec<Stack<'a>>,
}

// impl<'a> Stacks<'a> {
//     fn execute_command(&mut self, command: Move) {
//         println!("\n\n");
//         println!("Did I get updated? {:?}", &self);
//         println!("what length am I {:?}", &self.rows.len());
//         println!("The command from {:?}", command);
//         let move_from_index = (command.from - 1) as usize;
//         println!("Moving from {move_from_index}");
//         let move_to_index = (command.destination - 1) as usize;
//         println!("Moving to {move_to_index}");
//         let elements = command.movement_distance as usize;
//         println!("Moving X {elements}");
//         let mut stack_to_move_to = self.rows[move_to_index].clone();

//         let mut stack_to_move_from = self.rows[move_from_index].clone();
//         println!(
//             "Before {:?} and length {:?}",
//             &stack_to_move_from,
//             &stack_to_move_to.items.len()
//         );
//         let element_range_to_drain = 0..elements;
//         println!("The range {:?}", element_range_to_drain);
//         let mut elements_to_move = stack_to_move_from
//             .items
//             .drain(element_range_to_drain)
//             .collect::<Vec<_>>();

//         elements_to_move.reverse();
//         println!("Moving these elements in order of {:?}", &elements_to_move);
//         for (idx, item) in elements_to_move.iter().enumerate() {
//             stack_to_move_to.items.insert(idx, item);
//         }

//         self.rows[move_to_index] = stack_to_move_to;
//         self.rows[move_from_index] = stack_to_move_from;
//         println!("What am I now {:?}", &self);
//     }

//     fn get_top_rows(self) -> String {
//         let character_items = self
//             .rows
//             .iter()
//             .map(|stack| stack.get_top_item())
//             .collect::<Vec<_>>();
//         println!("The top rows {:?}", character_items);
//         String::from("Boo")
//     }
// }

// fn part_one_solution(input: &str) -> Result<String, Box<dyn Error>> {
//     let regex = Regex::new(r"\d+").unwrap();
//     let test = input.split("\n\n").collect::<Vec<_>>();
//     let mut board = test[0].split("\n").collect::<Vec<_>>();

//     println!("The board length {:?}", board.len());
//     let commands = test[1].split("\n").collect::<Vec<_>>();

//     println!("The commands {:?}", commands);
//     let parsed_commands = commands
//         .iter()
//         .map(|command| {
//             regex
//                 .find_iter(command)
//                 .filter_map(|digits| digits.as_str().parse::<u32>().ok())
//                 .collect::<Vec<_>>()
//         })
//         .map(|movement| {
//             println!("The movement {:?}", &movement);
//             Move {
//                 movement_distance: movement[0],
//                 from: movement[1],
//                 destination: movement[2],
//             }
//         })
//         .collect::<Vec<_>>();

//     println!("The board {:?}", &board);

//     board.pop();

//     let elements = board
//         .iter()
//         .map(|line| {
//             line.split("    ")
//                 .map(|filtered_lines| filtered_lines.split(" ").collect::<Vec<_>>())
//                 .flatten()
//                 .collect::<Vec<_>>()
//         })
//         .collect::<Vec<_>>();

//     println!("The elements {:?}", &elements);

//     let length = elements.len();

//     println!("The length {:?}", length);

//     let mut stacks = Vec::new();
//     for i in 0..=length {
//         println!("Where do I go up to {i}");
//         let relevant_items = elements.iter().map(|v| v[i]).collect::<Vec<_>>();
//         let stack = Stack {
//             items: relevant_items
//                 .into_iter()
//                 .filter(|l| !l.is_empty())
//                 .collect::<Vec<_>>(),
//         };
//         stacks.push(stack);
//     }

//     println!("The stacks {:?} and length {:?}", stacks, stacks.len());

//     let mut stacks = Stacks { rows: stacks };

//     for command in parsed_commands {
//         stacks.execute_command(command);
//     }
//     Ok(stacks.get_top_rows())
//     // Ok(String::from("Blooo"))
// }

impl<'a> Stacks<'a> {
    fn execute_command(&mut self, command: Move) {
        let move_from_index = (command.from - 1) as usize;
        let move_to_index = (command.destination - 1) as usize;
        let elements = command.movement_distance as usize;
        let mut stack_to_move_to = self.rows[move_to_index].clone();

        let mut stack_to_move_from = self.rows[move_from_index].clone();
        println!(
            "Before {:?} and length {:?}",
            &stack_to_move_from,
            &stack_to_move_to.items.len()
        );
        let element_range_to_drain = 0..elements;
        println!("The range {:?}", element_range_to_drain);
        let elements_to_move = stack_to_move_from
            .items
            .drain(element_range_to_drain)
            .collect::<Vec<_>>();

        if elements_to_move.len() > 1 {
            for (idx, item) in elements_to_move.iter().enumerate() {
                stack_to_move_to.items.insert(idx, item);
            }
        } else {
            stack_to_move_to.items.insert(0, elements_to_move[0]);
        }

        self.rows[move_to_index] = stack_to_move_to;
        self.rows[move_from_index] = stack_to_move_from;
    }

    fn get_top_rows(self) -> String {
        let character_items = self
            .rows
            .iter()
            .map(|stack| stack.get_top_item())
            .collect::<Vec<_>>();
        println!("The top rows {:?}", character_items);
        character_items
    }
}

fn part_two_solution(input: &str) -> Result<String, Box<dyn Error>> {
    let regex = Regex::new(r"\d+").unwrap();
    let test = input.split("\n\n").collect::<Vec<_>>();
    let mut board = test[0].split("\n").collect::<Vec<_>>();

    let commands = test[1].split("\n").collect::<Vec<_>>();

    let parsed_commands = commands
        .iter()
        .map(|command| {
            regex
                .find_iter(command)
                .filter_map(|digits| digits.as_str().parse::<u32>().ok())
                .collect::<Vec<_>>()
        })
        .map(|movement| {
            println!("The movement {:?}", &movement);
            Move {
                movement_distance: movement[0],
                from: movement[1],
                destination: movement[2],
            }
        })
        .collect::<Vec<_>>();

    println!("The board {:?}", &board);

    board.pop();

    let elements = board
        .iter()
        .map(|line| {
            line.split("    ")
                .map(|filtered_lines| filtered_lines.split(" ").collect::<Vec<_>>())
                .flatten()
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut stacks = Vec::new();
    for i in 0..=elements.len() {
        let relevant_items = elements.iter().map(|v| v[i]).collect::<Vec<_>>();
        let stack = Stack {
            items: relevant_items
                .into_iter()
                .filter(|l| !l.is_empty())
                .collect::<Vec<_>>(),
        };
        stacks.push(stack);
    }

    println!("The stacks {:?} and length {:?}", stacks, stacks.len());

    let mut stacks = Stacks { rows: stacks };

    for command in parsed_commands {
        stacks.execute_command(command);
    }
    Ok(stacks.get_top_rows())
}

// Supplies are stored in stacks of marked crates
// Crates need to be re-arranged
// The ship has Giant cargo crane capable of moving between stacks
// Forgot to ask which crate will end up where
// Do have a drawing of the starting stacks of crate and rearrangement process

// In the sample there are
// 3 stacks of crates
// Stack one has two crates, Z and (bottom - top)
// Commmands at bottom are moves
// Crates move one at a tmie (e.g. top becomes bottom if multiple are moved)

// Elves need to know which crate will end up on top of each stack and give this as input

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn part_one() {
    //     let input = include_str!("../sample.txt");
    //     let result = part_one_solution(input).unwrap();
    //     assert_eq!("0".to_owned(), result);
    // }

    #[test]
    // #[ignore]
    fn part_two() {
        let input = include_str!("../sample.txt");
        let result = part_two_solution(input).unwrap();
        assert_eq!("5".to_owned(), result);
    }
}
