mod get_monkey_values;
mod monkey;
mod parsing;

use get_monkey_values::get_highest_monkey_inspection_product_with_managed_stress;

fn main() {
    let input = include_str!("../input.txt");
    let part_one = get_highest_monkey_inspection_product_with_managed_stress(input, 20, 3);
    println!("The solution to part one {part_one}");
    let part_twp = get_highest_monkey_inspection_product_with_managed_stress(input, 10000, 1);
    println!("The solution to part one {part_twp}");
}

/*

Monkeys keep away with missing items
Need to predict where the monkeys will throw your items
Monkey operate on how worried I am about each item

Take some notes, puzzle input, about which item each monkey has, how worried you are about each item and how monkey makes a decision based on that


Starting level = worry level for each item monkey is currently holding, in the order they will be inspected
Operation = How the worry level changes as the monkey inspects an item (e.g. new = old * 5, worry level is 5x old where monkey inspects)
Test = Shows how the monkey uses the worry level to decide where to throw an item
    True = shows what happens to the item if test was true
    False = shows what happens to the item if test was false


After inspects but before test, your relief that the inspection didn't damage means worry level goes down by 3 (rounded down to nearest integer)

Turn = inspects and throws each item one by one

When monkey throws an item to another monkey, item goes on the end of the recipients lis
If a monkey holds no item, the turn ends

Focus on *two* most active monkeys
Count the total number of times each monkey inspects items over 20 rounds
Multiply the numbers together
*/
