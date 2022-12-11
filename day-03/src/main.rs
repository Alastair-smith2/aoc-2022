use std::collections::HashMap;

fn main() {
    let input = include_str!("../input.txt");
    let part_one = get_rucksack_duplicate_item_sum(input);
    println!("Part one result {part_one}");
    let part_two = get_elves_badge_sum(input);
    println!("Part two result {part_two}");
}

// Rucksacks with supplies
// Arranged wrong, needs re-arranging...
// Rucksacks have large compartments
// Items of a given type are meant to go into exactly one of the two compartments
// Elf failed to do to this for exactly one item type per rucksack

// Every item = lowercase letter...
// a and A refer to different type of items...
// List of items for a single rucksack = characters on one line
// first half of characters = items in first compartment

// example input has item that appears in both compartments

// Every item type has priority
// a-z = 1-26
// A-Z = 27-52

// part one = find the item types that appears in both compartments, what is the sum of the priorities

struct Rucksack {
    items: String,
    alphabet_with_value: HashMap<char, usize>,
}

#[derive(Debug)]
struct CharacterMap {
    character: char,
    value: u32,
}

impl Rucksack {
    pub fn new(items: String) -> Rucksack {
        Rucksack {
            items,
            alphabet_with_value: create_alphabet_map(),
        }
    }

    pub fn get_duplicate_value(&self) -> usize {
        let length = self.items.len();
        let (a, b) = self.items.split_at(length / 2);

        let mut comparetment_one_characters: Vec<char> = a.chars().collect();
        comparetment_one_characters.sort();
        comparetment_one_characters.dedup();
        let mut compartment_two_characters: Vec<char> = b.chars().collect();
        compartment_two_characters.sort();
        compartment_two_characters.dedup();

        let mut dup: Option<&char> = None;

        // Should be a far better way of doing this...
        comparetment_one_characters.iter().for_each(|c| {
            let duplicate = compartment_two_characters.contains(c);
            if duplicate {
                dup = Some(c)
            }
        });

        *self.alphabet_with_value.get(dup.unwrap()).unwrap()
    }

    // pub fn get_duplicate_value(&self) -> u32 {
    //     let length = self.items.len();
    //     let (a, b) = self.items.split_at(length / 2);

    //     let comparetment_one_characters: Vec<char> = a.chars().collect();
    //     let compartment_two_characters: Vec<char> = b.chars().collect();

    //     let result = comparetment_one_characters
    //         .iter()
    //         .find_map(|c| compartment_two_characters.iter().copied().find(|s| s == c))
    //         .expect("Should have had something");

    //     println!("result {:?}", result);
    // }
}

fn get_rucksack_duplicate_item_sum(input: &str) -> usize {
    input
        .split("\n")
        .map(|line| Rucksack::new(line.to_string()).get_duplicate_value())
        .sum()
}

fn get_elves_badge_sum(input: &str) -> usize {
    let alphabet = create_alphabet_map();
    let elves: Vec<&str> = input.split("\n").collect();
    let elf_groups = elves.chunks(3).collect::<Vec<_>>();
    let mut total_sum = 0;

    for group in elf_groups {
        let deduped = group
            .iter()
            .map(|s| s.chars().collect())
            .map(|mut item: Vec<char>| {
                item.sort();
                item.dedup();
                item
            })
            .collect::<Vec<_>>();

        let mut potential_elements = Vec::new();

        let elf_one = &deduped[0];
        let elf_two = &deduped[1];
        let elf_three = &deduped[2];

        // Should be a far better way of doing this...
        elf_one.iter().for_each(|c| {
            let duplicate = elf_two.contains(c);
            if duplicate {
                potential_elements.push(c)
            }
        });

        elf_three.iter().for_each(|c| {
            let duplicate = potential_elements.contains(&c);
            if duplicate {
                total_sum += alphabet.get(c).unwrap();
            }
        });
    }
    total_sum
}

fn create_alphabet_map() -> HashMap<char, usize> {
    ('a'..='z')
        .chain('A'..='Z')
        .enumerate()
        .map(|(idx, c)| (c, idx + 1))
        .collect::<HashMap<char, usize>>()
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn it_should_get_the_sum_of_priorities() {
        let input = include_str!("../sample.txt");
        let priority = get_rucksack_duplicate_item_sum(input);
        assert_eq!(157, priority);
    }

    #[test]
    fn it_should_get_the_sum_of_badges() {
        let input = include_str!("../sample.txt");
        let priority = get_elves_badge_sum(input);
        assert_eq!(70, priority);
    }
}

// part two
// elves are divided into groups of 3
// Each elf carries badge that identifies their group...
// badge is the only item type carried by all three elves
// E.g. all will have type B
// All badges need to be pulled
// Finding one item type that is common to all three elves
// Every set of three lines = single group
// item that appears in all three rucksacks
// Priorities
