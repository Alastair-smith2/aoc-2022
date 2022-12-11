use std::ops::RangeInclusive;

fn main() {
    let input = include_str!("../input.txt");
    let part_one_solution = find_number_of_ranges_by_filter(input, &range_contains_another);
    println!("Part one {part_one_solution}");
    let part_two_solution = find_number_of_ranges_by_filter(input, &ranges_overlap);
    println!("Part two {part_two_solution}");
}

// Section of camps
// Elves assigned job of cleaning up sections of the camp
// Each section has unique ID
// Many assignments overlap...
// Big list of section assignments for each pair

// 2-4 is inclusive range
// 2-3 is 2nd and 3rd section
// How many assignment pairs does one range fully contain the other

fn get_ranges(input: &str) -> Vec<Vec<RangeInclusive<usize>>> {
    input
        .split("\n")
        .map(|line| {
            line.split(",")
                .map(|element| {
                    let range_values = element.split("-").collect::<Vec<_>>();
                    let start = range_values[0].parse::<usize>().unwrap();
                    let end = range_values[1].parse::<usize>().unwrap();
                    let range = start..=end;
                    range
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn find_number_of_ranges_by_filter(
    input: &str,
    filter: &dyn Fn(&RangeInclusive<usize>, &RangeInclusive<usize>) -> bool,
) -> usize {
    let elve_ranges = get_ranges(input);
    elve_ranges
        .iter()
        .filter(|elements| filter(&elements[0], &elements[1]))
        .count()
}

fn ranges_overlap(range_one: &RangeInclusive<usize>, range_two: &RangeInclusive<usize>) -> bool {
    let (first_start, first_end, second_start, second_end) = range_points(range_one, range_two);
    first_start <= second_end && second_start <= first_end
}

fn range_contains_another(
    range_one: &RangeInclusive<usize>,
    range_two: &RangeInclusive<usize>,
) -> bool {
    let (first_start, first_end, second_start, second_end) = range_points(range_one, range_two);
    // Or is important here as we want to check whether each range is contained by the other...
    range_one.contains(second_start) && range_one.contains(second_end)
        || range_two.contains(first_start) && range_two.contains(first_end)
}

fn range_points<'a>(
    range_one: &'a RangeInclusive<usize>,
    range_two: &'a RangeInclusive<usize>,
) -> (&'a usize, &'a usize, &'a usize, &'a usize) {
    let first_start = range_one.start();
    let first_end = range_one.end();
    let second_start = range_two.start();
    let second_end = range_two.end();
    (first_start, first_end, second_start, second_end)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_get_range_overlap() {
        let range_one = 0..=10;
        let range_two = 1..=3;
        assert!(range_contains_another(&range_one, &range_two));
    }

    #[test]
    fn it_should_get_range_overlap_regardless_of_order() {
        let range_one = 1..=3;
        let range_two = 0..=10;
        assert!(range_contains_another(&range_one, &range_two));
    }

    #[test]
    fn it_should_not_get_overlaps_for_continuations() {
        let range_one = 0..=10;
        let range_two = 10..=20;
        assert_eq!(false, range_contains_another(&range_one, &range_two));
    }

    #[test]
    fn it_should_find_fully_inclusive_ranges() {
        let input = include_str!("../sample.txt");
        let number_of_overlaps = find_number_of_ranges_by_filter(input, &range_contains_another);
        assert_eq!(2, number_of_overlaps);
    }

    #[test]
    fn it_should_find_overlapping_ranges() {
        let input = include_str!("../sample.txt");
        let number_of_overlaps = find_number_of_ranges_by_filter(input, &ranges_overlap);
        assert_eq!(4, number_of_overlaps);
    }
}
