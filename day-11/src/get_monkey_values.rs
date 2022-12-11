use crate::parsing::parse_monkey;

#[derive(Debug)]
struct Destination {
    value: u64,
    id: u32,
}

pub fn get_highest_monkey_inspection_product_with_managed_stress(
    input: &str,
    number_of_steps: u64,
    worry_level_factor: u64,
) -> u64 {
    let input_of_monkeys = input.split("\n\n").collect::<Vec<_>>();
    let mut monkeys = input_of_monkeys
        .iter()
        .map(|m| parse_monkey(m).unwrap().1)
        .collect::<Vec<_>>();

    for _ in 1..=number_of_steps {
        let mut values = vec![];
        for worry_idx in 0..monkeys.len() {
            values.push(*&monkeys[worry_idx].test_factor);
        }
        let stress_management: u64 = values.iter().product();

        for idx in 0..monkeys.len() {
            let mut items_to_move: Vec<Destination> = vec![];

            for item_idx in 0..monkeys[idx].items.len() {
                let monkey_inspecting_worry_level =
                    monkeys[idx].get_worry_level_for_item(monkeys[idx].items[item_idx]);
                let current_worry_level =
                    (monkey_inspecting_worry_level / worry_level_factor) % stress_management;
                let destination = monkeys[idx].find_monkey_to_throw_to(current_worry_level);
                items_to_move.push(Destination {
                    value: current_worry_level,
                    id: destination,
                });
                monkeys[idx].inspection_count += 1;
            }
            monkeys[idx].items.clear();
            for item in items_to_move {
                monkeys[item.id as usize].items.push(item.value)
            }
        }
    }

    monkeys.sort_by(|a, b| b.inspection_count.cmp(&a.inspection_count));
    let highest_monkeys_value = monkeys
        .iter()
        .take(2)
        .map(|m| m.inspection_count)
        .product::<u64>();
    highest_monkeys_value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one() {
        let input = include_str!("../sample.txt");
        let result = get_highest_monkey_inspection_product_with_managed_stress(input, 20, 3);
        assert_eq!(10605, result);
    }

    #[test]
    fn part_two() {
        let input = include_str!("../sample.txt");
        let result = get_highest_monkey_inspection_product_with_managed_stress(input, 1, 1);
        assert_eq!(24, result);
    }

    #[test]
    fn part_two_20_round() {
        let input = include_str!("../sample.txt");
        let result = get_highest_monkey_inspection_product_with_managed_stress(input, 20, 1);
        assert_eq!(10197, result);
    }

    #[test]
    fn part_two_1000_round() {
        let input = include_str!("../sample.txt");
        let result = get_highest_monkey_inspection_product_with_managed_stress(input, 1000, 1);
        assert_eq!(27019168, result);
    }
}
