fn main() {
    println!("Hello, world!");
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
        let result = some_func(&input);
        assert_eq!(2, result)
    }

    #[test]
    fn part_two() {
        let input = include_str!("../sample.txt");
        let result = some_func(&input);
        assert_eq!(5, result)
    }
}
