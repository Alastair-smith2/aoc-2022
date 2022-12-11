// Not working yet, still WIP

use crate::parse::*;

#[derive(Debug, PartialEq, Clone)]
struct Coordinate {
    y: usize,
    x: usize,
}

impl Coordinate {
    fn get_starting_position(grid_height: usize) -> Self {
        Coordinate { x: 12, y: 16 }
    }

    fn other_coordinate_on_same_column(&self, knot: &Self) -> bool {
        self.x == knot.x
    }

    fn other_coordinate_on_same_row(&self, knot: &Self) -> bool {
        self.y == knot.y
    }

    fn should_knot_move(&self, knot: &Self) -> bool {
        let same_column = match self.x.checked_sub(knot.x) {
            Some(val) => val > 1,
            None => match knot.x.checked_sub(self.x) {
                Some(val) => val > 1,
                None => false,
            },
        };
        let same_row = match self.y.checked_sub(knot.y) {
            Some(val) => val > 1,
            None => match knot.y.checked_sub(self.y) {
                Some(val) => val > 1,
                None => false,
            },
        };
        same_column || same_row
    }
}
#[derive(Debug)]
struct Rope {
    knots: Vec<Coordinate>,
    visited_coordinates: Vec<Coordinate>,
    size: usize,
}

impl Rope {
    fn new(grid_height: usize, size: usize) -> Self {
        let knots = (0..10)
            .map(|i| Coordinate::get_starting_position(grid_height))
            .collect::<Vec<_>>();
        let visited_coordinates = vec![Coordinate::get_starting_position(grid_height)];
        Rope {
            knots,
            visited_coordinates,
            size,
        }
    }

    fn move_direction(&mut self, direction: &Direction) {
        match direction {
            Direction::Up(distance) => {
                let range = 0..*distance;
                for _ in range {
                    self.knots[0].y -= 1;

                    for knot_idx in 1..self.knots.len() {
                        if self.knots[knot_idx]
                            .other_coordinate_on_same_column(&self.knots[knot_idx - 1])
                            && self.knots[knot_idx].should_knot_move(&self.knots[knot_idx - 1])
                        {
                            self.knots[knot_idx].y -= 1;
                            if knot_idx == self.size - 1 {
                                let new_coordinate = self.knots[knot_idx].clone();
                                if self.new_coordinate_needs_to_be_added(&new_coordinate) {
                                    self.visited_coordinates.push(new_coordinate);
                                }
                            }
                        }

                        if self.knots[knot_idx].should_knot_move(&self.knots[knot_idx - 1])
                            && !self.knots[knot_idx]
                                .other_coordinate_on_same_row(&self.knots[knot_idx - 1])
                        {
                            self.knots[knot_idx].y = self.knots[knot_idx - 1].y + 1;
                            self.knots[knot_idx].x = self.knots[knot_idx - 1].x;

                            if knot_idx == self.size - 1 {
                                let new_coordinate = self.knots[knot_idx].clone();
                                if self.new_coordinate_needs_to_be_added(&new_coordinate) {
                                    self.visited_coordinates.push(new_coordinate);
                                }
                            }
                        }
                    }
                }
            }
            Direction::Down(distance) => {
                let range = 0..*distance;
                for _ in range {
                    self.knots[0].y += 1;

                    for knot_idx in 1..self.knots.len() {
                        if self.knots[knot_idx]
                            .other_coordinate_on_same_column(&self.knots[knot_idx - 1])
                            && self.knots[knot_idx].should_knot_move(&self.knots[knot_idx - 1])
                        {
                            self.knots[knot_idx].y += 1;

                            self.add_visited_if_last_element(knot_idx);
                        }

                        if self.knots[knot_idx].should_knot_move(&self.knots[knot_idx - 1])
                            && !self.knots[knot_idx]
                                .other_coordinate_on_same_row(&self.knots[knot_idx - 1])
                        {
                            self.knots[knot_idx].y = self.knots[knot_idx - 1].y - 1;
                            self.knots[knot_idx].x = self.knots[knot_idx - 1].x;

                            self.add_visited_if_last_element(knot_idx);
                        }
                    }
                }
                // }
            }
            Direction::Left(distance) => {
                let range = 0..*distance;
                for _ in range {
                    self.knots[0].x -= 1;

                    for knot_idx in 1..self.knots.len() {
                        if self.knots[knot_idx]
                            .other_coordinate_on_same_row(&self.knots[knot_idx - 1])
                            && self.knots[knot_idx].should_knot_move(&self.knots[knot_idx - 1])
                        {
                            self.knots[knot_idx].x -= 1;

                            self.add_visited_if_last_element(knot_idx);
                        }

                        if self.knots[knot_idx].should_knot_move(&self.knots[knot_idx - 1])
                            && !self.knots[knot_idx]
                                .other_coordinate_on_same_column(&self.knots[knot_idx - 1])
                        {
                            self.knots[knot_idx].x = self.knots[knot_idx - 1].x + 1;
                            self.knots[knot_idx].y = self.knots[knot_idx - 1].y;

                            self.add_visited_if_last_element(knot_idx);
                        }
                    }
                }
            }
            Direction::Right(distance) => {
                let range = 0..*distance;
                for _ in range {
                    self.knots[0].x += 1;

                    for knot_idx in 1..self.knots.len() {
                        if self.knots[knot_idx]
                            .other_coordinate_on_same_row(&self.knots[knot_idx - 1])
                            && self.knots[knot_idx].should_knot_move(&self.knots[knot_idx - 1])
                        {
                            self.knots[knot_idx].x += 1;

                            self.add_visited_if_last_element(knot_idx);
                        }

                        if self.knots[knot_idx].should_knot_move(&self.knots[knot_idx - 1])
                            && !self.knots[knot_idx]
                                .other_coordinate_on_same_column(&self.knots[knot_idx - 1])
                        {
                            self.knots[knot_idx].x = self.knots[knot_idx - 1].x - 1;
                            self.knots[knot_idx].y = self.knots[knot_idx - 1].y;

                            self.add_visited_if_last_element(knot_idx);
                        }
                    }
                }
                // }
            }
        }
    }

    // This may be simpler with false matrix...
    fn new_coordinate_needs_to_be_added(&self, coordinate: &Coordinate) -> bool {
        !self.visited_coordinates.contains(coordinate)
    }

    fn add_visited_if_last_element(&mut self, knot_idx: usize) {
        if knot_idx == self.size - 1 {
            self.add_element(knot_idx);
        }
    }

    fn add_element(&mut self, knot_idx: usize) {
        let new_coordinate = self.knots[knot_idx].clone();
        if self.new_coordinate_needs_to_be_added(&new_coordinate) {
            self.visited_coordinates.push(new_coordinate);
        }
    }
}

pub fn part_two_solution(input: &str) -> usize {
    let directions = input
        .lines()
        .map(|l| parse_movements(l).unwrap().1)
        .collect::<Vec<_>>();

    let mut rope = Rope::new(26, 10);
    for direction in directions {
        rope.move_direction(&direction);
        // println!("The rope {:?} after {:?}", &rope, &direction);
    }

    println!("The visited coordinates {:?}", &rope.visited_coordinates);

    rope.visited_coordinates.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_two() {
        let input = include_str!("../part_two_sample.txt");
        let result = part_two_solution(input);
        assert_eq!(36, result);
    }
}

// Tail can only move up or down if we've moved left / right and not on same column?
// Tail can only move left / right if we've moved up / down and not on same row?

// Key thing here is that we need to
// Apply the logic once to each knot PER step
// Need to check the knot ahead

// TODO split distance into groups thaat then get applied to the vec
// Eg. 17 should be 10 and 7 (based on 10)

// Movement is wrong,
// Need to move each node by the step...
// But then also need to chunk because I only have 10 elements

// I'm currently only moving each knot once by the movement, rather than rest
// Would it work if I focused just moving the head, but then check the remaining element after the next? (e.g. is the second close enough to the first, then is the third close enough to the second?)
