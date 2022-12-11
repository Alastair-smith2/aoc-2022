mod part_one;
mod part_two;

use part_one::get_total_trees_visible;
use part_two::check_most_scenic_tree;

fn main() {
    println!("Hello, world!");
    let input = include_str!("../input.txt");
    let part_one_solution = get_total_trees_visible(input);
    println!("Part one solution {part_one_solution}");
    let part_two_solution = check_most_scenic_tree(input);
    println!("Part two solution {part_two_solution}");
}
/*
Treehouse

Enough trees to keep a tree house hidden?
Each tree = number that represents height
Tree is visible if all the other trees between it and the edge of the grid are shorter than it
Only consider trees in the same row or column...
Only look up, down, left, right from any given tree
All of trees around the edge are visible
Only interior trees to consider

How many trees are visible from outside the grid
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one() {
        let input = include_str!("../sample.txt");
        let result = get_total_trees_visible(input);
        assert_eq!(21, result);
    }

    /*
    See lots of trees

    */
    #[test]
    fn part_two() {
        let input = include_str!("../sample.txt");
        let result = check_most_scenic_tree(input);
        assert_eq!(8, result);
    }
}
