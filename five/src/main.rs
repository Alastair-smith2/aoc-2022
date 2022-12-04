fn main() {
    let input = include_str!("../input.txt");
    let part_one_solution = some_func(input);
    println!("Part one {part_one_solution}");
    let part_two_solution = some_func(input);
    println!("Part two {part_two_solution}");
}

fn some_func(input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one() {
        let input = include_str!("../sample.txt");
        let result = some_func(input);
        assert_eq!(2, result);
    }

    #[test]
    #[ignore]
    fn part_two() {
        let input = include_str!("../sample.txt");
        let result = some_func(input);
        assert_eq!(5, result);
    }
}
