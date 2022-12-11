use std::{
    collections::{HashSet, VecDeque},
    error::Error,
};

extern crate nom;

fn main() -> Result<(), Box<dyn Error>> {
    let input = include_str!("../input.txt");
    let part_one_solution = find_start_position(input, 4);
    println!("Part one {part_one_solution}");
    let part_two_solution = find_start_position(input, 14);
    println!("Part two {part_two_solution}");
    Ok(())
}

/*
star fruit grove
Handheld device, communication device
Fixing a malfunctioning one?
Colorful sparks...
Need to lock on to their signal
Detects start of packet marker

Start of packet is indicated by sequence of four characters that are all different...

Find first position where the four most recent characters received were all different.

First time the marker appears is after the 7th character, the 4 characters before were jpqm

 */

fn find_start_position(input: &str, unique_size: usize) -> usize {
    let chars = input.chars().collect::<VecDeque<_>>();
    let mut initial_input = chars.iter().take(3).collect::<VecDeque<_>>();
    let mut count = 0;
    for (idx, char) in chars.iter().enumerate().skip(3) {
        initial_input.push_back(char);
        let mut unique_values: HashSet<&char> = HashSet::new();
        unique_values.extend(initial_input.iter());
        if unique_values.len() == unique_size {
            count = idx + 1;
            break;
        }
        if initial_input.len() == unique_size {
            initial_input.pop_front();
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_sample_one() {
        let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        let result = find_start_position(input, 4);
        assert_eq!(7, result);
    }

    #[test]
    fn part_one_sample_two() {
        let input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        let result = find_start_position(input, 4);
        assert_eq!(5, result);
    }

    #[test]
    fn part_one_sample_three() {
        let input = "nppdvjthqldpwncqszvftbrmjlhg";
        let result = find_start_position(input, 4);
        assert_eq!(6, result);
    }

    #[test]
    fn part_one_sample_four() {
        let input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        let result = find_start_position(input, 4);
        assert_eq!(10, result);
    }

    #[test]
    fn part_one_sample_five() {
        let input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        let result = find_start_position(input, 4);
        assert_eq!(11, result);
    }

    #[test]
    fn part_two_sample_one() {
        let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        let result = find_start_position(input, 14);
        assert_eq!(19, result);
    }

    #[test]
    fn part_two_sample_two() {
        let input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        let result = find_start_position(input, 14);
        assert_eq!(23, result);
    }

    #[test]
    fn part_two_sample_three() {
        let input = "nppdvjthqldpwncqszvftbrmjlhg";
        let result = find_start_position(input, 14);
        assert_eq!(23, result);
    }

    #[test]
    fn part_two_sample_four() {
        let input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        let result = find_start_position(input, 14);
        assert_eq!(29, result);
    }

    #[test]
    fn part_two_sample_five() {
        let input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        let result = find_start_position(input, 14);
        assert_eq!(26, result);
    }
}
