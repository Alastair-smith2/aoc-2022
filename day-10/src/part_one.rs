use crate::input::{get_input, Command};

fn get_cycle_value(current_cycle_step: usize, register: i64) -> i64 {
    match current_cycle_step {
        20 => {
            println!("current value at 20th {register}");
            register * 20
        }
        60 => {
            println!("current value at 60th {register}");
            register * 60
        }
        100 => {
            println!("current value at 100th {register}");
            register * 100
        }
        140 => {
            println!("current value at 140th {register}");
            register * 140
        }
        180 => {
            println!("current value at 180th {register}");
            register * 180
        }
        220 => {
            println!("current value at 220th {register}");
            register * 220
        }
        _ => 0,
    }
}

pub fn get_signal_strength(input: &str) -> i64 {
    let commands = get_input(input);
    let mut values: Vec<i64> = vec![];

    let mut register = 1;

    let mut current_cycle_step = 1;

    for command in commands {
        println!("Cycle count {current_cycle_step} and command {command:?}");
        match command {
            Command::Noop => {
                values.push(get_cycle_value(current_cycle_step, register));
                current_cycle_step += 1;
            }
            Command::AddX(instruction) => {
                values.push(get_cycle_value(current_cycle_step, register));
                current_cycle_step += 1;
                values.push(get_cycle_value(current_cycle_step, register));
                current_cycle_step += 1;
                register += instruction
            }
        }
    }

    values.iter().sum()
}

/*

Cathode ray tube (replacement for device's video system)
Clock circuit
Ticks at a certain rate; each tick = a cycle
Single register X, starts with value of 1

Supports two instructions
1) addx <Value> - takes two cycles to complete, after two cycles X is increased by value of <value> (can be negative)
2) noop - takes one cycle to complete, no effect

Tells the screen what to draw

Find signal strength at 20th, 60th, 140th, 180th and 220th cycle

Sum is current value X <cycle time> (e.g. 21 * 20)


noop
addx 3
addx -5

Cycle 1 = 1
Cycle 2 = 1
Cycle 3 = 1
Cycle 4 = 5
Cycle 5 - 1


 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one() {
        let input = include_str!("../sample.txt");
        let result = get_signal_strength(input);
        assert_eq!(13140, result);
    }
}
