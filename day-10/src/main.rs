mod input;
mod part_one;
mod part_two;

use part_one::get_signal_strength;

use crate::part_two::get_crt;

fn main() {
    let input = include_str!("../input.txt");
    let part_one = get_signal_strength(input);
    println!("The solution to part one {part_one}");
    let part_two = get_crt(input);
    println!("The solution to part one {part_two}");
}
