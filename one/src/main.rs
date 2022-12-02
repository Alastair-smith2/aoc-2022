use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let part_one = get_highest_elf_calories(&contents);
    println!("Part one answer -  {part_one}");
    let highest_elves_calories = get_top_three_elves_calories(&contents);
    println!("Part two answer -  {highest_elves_calories}");
}

fn group_calories(contents: &String) -> Vec<i32> {
    let result = contents
        .split("\n\n")
        .map(|l| l.split("\n").map(|s| s.parse::<i32>().unwrap_or(0)).sum())
        .collect();

    result
}

fn get_highest_elf_calories(contents: &String) -> i32 {
    let count_list = group_calories(contents);
    *count_list.iter().max().unwrap()
}

fn get_top_three_elves_calories(contents: &String) -> i32 {
    let mut count_list = group_calories(contents);
    count_list.sort_by(|a, b| b.cmp(a));
    count_list.iter().take(3).sum::<i32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_find_highest_calories_for_an_elft() {
        let contents =
            fs::read_to_string("sample_input.txt").expect("Should have been able to read the file");
        let calories = get_highest_elf_calories(&contents);
        assert_eq!(24000, calories);
    }

    #[test]
    fn it_should_find_highest_calories_if_greatest_value_is_last_line() {
        let input = String::from("1000\n2000\n\n3000\n5000\n\n100000");
        let calories = get_highest_elf_calories(&input);
        assert_eq!(100000, calories);
    }

    #[test]
    fn it_should_find_top_three_calories() {
        let contents =
            fs::read_to_string("sample_input.txt").expect("Should have been able to read the file");
        let calories = get_top_three_elves_calories(&contents);
        assert_eq!(45000, calories);
    }
}
