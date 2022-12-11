mod parse;
mod part_one;
mod part_two;

use part_one::part_one_solution;
use part_two::part_two_solution;

fn main() {
    let input = include_str!("../input.txt");
    let part_one_solution = part_one_solution(input);
    println!("Part one solution {part_one_solution}");
    // let part_two_solution = part_two_solution(input);
    // println!("Part two solution {part_two_solution}");
}
