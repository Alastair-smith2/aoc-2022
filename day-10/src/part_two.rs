/*
X controls horizontal position of a sprite
sprite = 3 pixels wide

X register sets horizontal position of the middle of that sprite (no vertical in this)

Screen draws top row, left to right

40 wide, 6 high
Left most pixel = position 0
Right more pixel = position 39

CPU tied closely to the clock circuit
CRT draws *a single pixel each cycle*
Each pixel as a #

Should be able to determine whether the sprite is visible the instant each pixel is drawn

If sprite is positioned so that one of its three pixels is the pixel currently being drawn, produces a lit #
Otherwise has a .
*/

use std::ops::RangeInclusive;

use crate::input::{get_input, Command};

fn get_drawing<'a>(register: i64, current_cycle_step: i64) -> &'a str {
    let range: RangeInclusive<i64> = register - 1..=register + 1;
    if range.contains(&current_cycle_step) {
        "#"
    } else {
        "."
    }
}

pub fn get_crt(input: &str) -> usize {
    let commands = get_input(input);

    let mut register = 1;

    let mut current_cycle_step = 0;

    let mut crt: Vec<&str> = vec![];

    for command in commands {
        match command {
            Command::Noop => {
                crt.push(get_drawing(register, current_cycle_step % 40));
                current_cycle_step += 1;
            }
            Command::AddX(instruction) => {
                crt.push(get_drawing(register, current_cycle_step % 40));
                current_cycle_step += 1;
                crt.push(get_drawing(register, current_cycle_step % 40));
                current_cycle_step += 1;
                register += instruction;
            }
        }
    }

    let result = crt.chunks(40).collect::<Vec<_>>();
    println!("The result {:?}", result);

    0
}

// ["#", "#", ".", ".", "#", "#", ".", ".", "#", "#", ".", ".", "#", "#", ".", ".", "#", "#", ".", ".", "#", "#", ".", ".", "#", "#", ".", ".", "#", "#", ".", ".", "#", "#", ".", ".", "#", "#", ".", "."]
// ["#", "#", "#", ".", ".", ".", "#", "#", "#", ".", ".", ".", "#", "#", "#", ".", ".", ".", "#", "#", "#", ".", ".", ".", "#", "#", "#", ".", ".", ".", "#", "#", "#", ".", ".", ".", "#", "#", "#", "."]
// ["#", "#", "#", "#", ".", ".", ".", ".", "#", "#", "#", "#", ".", ".", ".", ".", "#", "#", "#", "#", ".", ".", ".", ".", "#", "#", "#", "#", ".", ".", ".", ".", "#", "#", "#", "#", ".", ".", ".", "."]
// ["#", "#", "#", "#", "#", ".", ".", ".", ".", ".", "#", "#", "#", "#", "#", ".", ".", ".", ".", ".", "#", "#", "#", "#", "#", ".", ".", ".", ".", ".", "#", "#", "#", "#", "#", ".", ".", ".", ".", "."]
// ["#", "#", "#", "#", "#", "#", ".", ".", ".", ".", ".", ".", "#", "#", "#", "#", "#", "#", ".", ".", ".", ".", ".", ".", "#", "#", "#", "#", "#", "#", ".", ".", ".", ".", ".", ".", "#", "#", "#", "#"]
// ["#", "#", "#", "#", "#", "#", "#", ".", ".", ".", ".", ".", ".", ".", "#", "#", "#", "#", "#", "#", "#", ".", ".", ".", ".", ".", ".", ".", "#", "#", "#", "#", "#", "#", "#", ".", ".", ".", ".", "."]
