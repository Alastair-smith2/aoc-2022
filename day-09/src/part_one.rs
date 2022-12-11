/*
Rope bridge

Figure out where not to step
Knots mark the head and tail of the rope, if head moves far enough away, tail is pulled toward the head
2D grid
Hypothecitcal series of motions (puzzle input) should be able to determine how the tail will move

Rope must always be quite short, Head and tail must always be touching (diagonally adjacent and even overlapping both count at touching)
E.g.

If head is ever two steps directly up, down, left or right from tail, tail must also move in that direction so it remains close enough
.....    .....    .....
.TH.. -> .T.H. -> ..TH.
.....    .....    .....

...    ...    ...
.T.    .T.    ...
.H. -> ... -> .T.
...    .H.    .H.
...    ...    ...

Otherwise tail move diagonally one step to keep up
.....    .....    .....
.....    ..H..    ..H..
..H.. -> ..... -> ..T..
.T...    .T...    .....
.....    .....    .....

.....    .....    .....
.....    .....    .....
..H.. -> ...H. -> ..TH.
.T...    .T...    .....
.....    .....    .....

Need to work out where the tail goes as the head follows a series of motions.
Assume head and tail both start at the same position


How many positions does the tail of the rope visit at least once
*/

use crate::parse::{parse_movements, Direction};

#[derive(Debug, PartialEq, Clone)]
struct Coordinate {
    y: usize,
    x: usize,
}

impl Coordinate {
    fn get_starting_position(grid_height: usize) -> Self {
        Coordinate {
            x: grid_height / 2,
            y: grid_height - 1,
        }
    }
}

#[derive(Debug)]
struct Rope {
    start_position: Coordinate,
    head_position: Coordinate,
    tail_position: Coordinate,
    visited_coordinates: Vec<Coordinate>,
}

impl Rope {
    fn new(grid_height: usize) -> Self {
        let start_position = Coordinate::get_starting_position(grid_height);
        let head_position = Coordinate::get_starting_position(grid_height);
        let tail_position = Coordinate::get_starting_position(grid_height);
        let visited_coordinates = vec![Coordinate::get_starting_position(grid_height)];
        Rope {
            start_position,
            head_position,
            tail_position,
            visited_coordinates,
        }
    }

    fn move_direction(&mut self, direction: &Direction) {
        match direction {
            Direction::Up(distance) => {
                let distance_range = 0..*distance;
                for _ in distance_range {
                    self.head_position.y -= 1;

                    if self.tail_and_head_are_on_same_column() && self.should_tail_move() {
                        println!("The tail should move on the same column");
                        self.tail_position.y -= 1;

                        let new_coordinate = self.tail_position.clone();
                        if self.new_coordinate_needs_to_be_added(&new_coordinate) {
                            self.visited_coordinates.push(new_coordinate);
                        }
                    }

                    if self.should_tail_move() && !self.tail_and_head_are_on_same_row() {
                        // we should always move the tail_position.<val> to what the value was originally
                        self.tail_position.y = self.head_position.y + 1;
                        self.tail_position.x = self.head_position.x;

                        let new_coordinate = self.tail_position.clone();
                        if self.new_coordinate_needs_to_be_added(&new_coordinate) {
                            self.visited_coordinates.push(new_coordinate);
                        }
                    }
                }
            }
            Direction::Down(distance) => {
                let distance_range = 0..*distance;
                for _ in distance_range {
                    self.head_position.y += 1;

                    if self.tail_and_head_are_on_same_column() && self.should_tail_move() {
                        println!("The tail should move on the same column");
                        self.tail_position.y += 1;

                        let new_coordinate = self.tail_position.clone();
                        if self.new_coordinate_needs_to_be_added(&new_coordinate) {
                            self.visited_coordinates.push(new_coordinate);
                        }
                    }

                    if self.should_tail_move() && !self.tail_and_head_are_on_same_row() {
                        self.tail_position.y = self.head_position.y - 1;
                        self.tail_position.x = self.head_position.x;

                        let new_coordinate = self.tail_position.clone();
                        if self.new_coordinate_needs_to_be_added(&new_coordinate) {
                            self.visited_coordinates.push(new_coordinate);
                        }
                    }
                }
            }
            Direction::Left(distance) => {
                let distance_range = 0..*distance;
                for _ in distance_range {
                    self.head_position.x -= 1;
                    if self.tail_and_head_are_on_same_row() && self.should_tail_move() {
                        self.tail_position.x -= 1;

                        let new_coordinate = self.tail_position.clone();
                        if self.new_coordinate_needs_to_be_added(&new_coordinate) {
                            self.visited_coordinates.push(new_coordinate);
                        }
                    }

                    if self.should_tail_move() && !self.tail_and_head_are_on_same_column() {
                        self.tail_position.x = self.head_position.x + 1;
                        self.tail_position.y = self.head_position.y;

                        let new_coordinate = self.tail_position.clone();
                        if self.new_coordinate_needs_to_be_added(&new_coordinate) {
                            self.visited_coordinates.push(new_coordinate);
                        }
                    }
                }
            }
            Direction::Right(distance) => {
                let distance_range = 0..*distance;
                for _ in distance_range {
                    self.head_position.x += 1;
                    if self.tail_and_head_are_on_same_row() && self.should_tail_move() {
                        self.tail_position.x += 1;

                        let new_coordinate = self.tail_position.clone();
                        if self.new_coordinate_needs_to_be_added(&new_coordinate) {
                            self.visited_coordinates.push(new_coordinate);
                        }
                    }

                    if self.should_tail_move() && !self.tail_and_head_are_on_same_column() {
                        // TODO CHECK THIS
                        self.tail_position.x = self.head_position.x - 1;
                        self.tail_position.y = self.head_position.y;

                        let new_coordinate = self.tail_position.clone();
                        if self.new_coordinate_needs_to_be_added(&new_coordinate) {
                            self.visited_coordinates.push(new_coordinate);
                        }
                    }
                }
            }
        }
    }

    fn tail_and_head_are_on_same_row(&self) -> bool {
        self.head_position.y == self.tail_position.y
    }

    fn tail_and_head_are_on_same_column(&self) -> bool {
        self.head_position.x == self.tail_position.x
    }

    fn should_tail_move(&self) -> bool {
        println!(
            "The head, x position {:?}, y position {:?}",
            &self.head_position.x, &self.head_position.y
        );
        println!(
            "The tail, x position {:?}, y position {:?}",
            &self.tail_position.x, &self.tail_position.y
        );

        // If we're two steps away on the same column
        // self.head_position.x - self.tail_position.x > 1
        // If we're two steps away on the same row
        // || self.tail_position.y - self.head_position.y > 1
        let same_column = match self.head_position.x.checked_sub(self.tail_position.x) {
            Some(val) => val > 1,
            None => match self.tail_position.x.checked_sub(self.head_position.x) {
                Some(val) => val > 1,
                None => false,
            },
        };
        let same_row = match self.tail_position.y.checked_sub(self.head_position.y) {
            Some(val) => val > 1,
            None => match self.head_position.y.checked_sub(self.tail_position.y) {
                Some(val) => val > 1,
                None => false,
            },
        };
        same_column || same_row
    }

    fn new_coordinate_needs_to_be_added(&self, coordinate: &Coordinate) -> bool {
        !self.visited_coordinates.contains(coordinate)
    }

    // Need to determine whether it's row or column that's different
    // So left or right mean could be a change in column
    // Up or down mean could be a change in row

    // we can only move diagonally right if the direction was up

    // move diagonally and right
    // Coordinate { x: 4, y: 2 } Coordinate { x: 3, y: 4 }

    // difference of x 2 = left or right
    // head position  x >2 = left
    // tail poisition x <2 = right

    // whether it's up or down will be if y has a difference of 1
    // up = -1
    // down = +1
    // Move diagonally up and left Head Coordinate { x: 2, y: 0 } Tail Coordinate { x: 4, y: 1 }
    // Move diagonally up and right Head Coordinate { x: 4, y: 0 } Tail Coordinate { x: 2, y: 1 }
}

pub fn part_one_solution(input: &str) -> usize {
    let directions = input
        .lines()
        .map(|l| parse_movements(l).unwrap().1)
        .collect::<Vec<_>>();

    let mut rope = Rope::new(3000);
    for direction in directions {
        println!("Should move... {:?}", direction);
        rope.move_direction(&direction);
    }
    println!("The rope {:?}", &rope.visited_coordinates);
    rope.visited_coordinates.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one() {
        let input = include_str!("../sample.txt");
        let result = part_one_solution(input);
        assert_eq!(13, result);
    }
}
