fn main() {
    let input = include_str!("../input.txt");

    let part_one = calculate_result(input);
    println!("Part one {part_one}");
    let part_two = calculate_correct_score(input);
    println!("Part two {part_two}");
}

enum DesiredResult {
    Lose(u32),
    Draw(u32),
    Win(u32),
}

enum ElfChoice {
    Rock,
    Paper,
    Scissors,
}

const ROCK_VALUE: u32 = 1;
const PAPER_VALUE: u32 = 2;
const SCISSOR_VALUE: u32 = 3;

const LOST_SCORE: u32 = 0;
const DRAW_SCORE: u32 = 3;
const WIN_SCORE: u32 = 6;

// First column is what the oponent plays
// A = rock
// B = Paper
// C = Scissors

// Second column = what you should play in response
// X = rock
// Y = Paper
// Z = scissors
// Must not win everytime

// My total score = sum of my score for each round
// Score for a round =
// Shape selected (1 rock, 2 paper, 3 scissors)
// AND
// Score for the outcome (0 lost, 3 draw, 6 win)

fn calculate_result(input: &str) -> u32 {
    input
        .split("\n")
        .map(|l| l.split(" ").collect())
        .map(|hand: Vec<&str>| {
            let hand_pair: (&str, &str) = (hand[0], hand[1]);

            match hand_pair {
                ("A", "X") => ROCK_VALUE + DRAW_SCORE,
                ("A", "Y") => PAPER_VALUE + WIN_SCORE,
                ("A", "Z") => SCISSOR_VALUE + LOST_SCORE,
                //
                ("B", "X") => ROCK_VALUE + LOST_SCORE,
                ("B", "Y") => PAPER_VALUE + DRAW_SCORE,
                ("B", "Z") => SCISSOR_VALUE + WIN_SCORE,
                //
                ("C", "X") => ROCK_VALUE + WIN_SCORE,
                ("C", "Y") => PAPER_VALUE + LOST_SCORE,
                ("C", "Z") => SCISSOR_VALUE + DRAW_SCORE,
                _ => {
                    panic!("Should not have reached this combination")
                }
            }
        })
        .sum()

    // Original
    // let mut total_score = 0;
    // for line in input.lines() {
    //     let hand_choices: Vec<&str> = line.split(" ").collect();
    //     let hand_pair: (&str, &str) = (hand_choices[0], hand_choices[1]);

    //     let hand_result = match hand_pair {
    //         ("A", "X") => ROCK_VALUE + DRAW_SCORE,
    //         ("A", "Y") => PAPER_VALUE + WIN_SCORE,
    //         ("A", "Z") => SCISSOR_VALUE + LOST_SCORE,
    //         //
    //         ("B", "X") => ROCK_VALUE + LOST_SCORE,
    //         ("B", "Y") => PAPER_VALUE + DRAW_SCORE,
    //         ("B", "Z") => SCISSOR_VALUE + WIN_SCORE,
    //         //
    //         ("C", "X") => ROCK_VALUE + WIN_SCORE,
    //         ("C", "Y") => PAPER_VALUE + LOST_SCORE,
    //         ("C", "Z") => SCISSOR_VALUE + DRAW_SCORE,
    //         _ => {
    //             panic!("Should not have reached this combination")
    //         }
    //     };
    //     total_score += hand_result;
    // }
    // total_score
}

fn calculate_correct_score(input: &str) -> u32 {
    input
        .split("\n")
        .map(|l| l.split(" ").collect())
        .map(|hand: Vec<&str>| {
            let elf_choice: ElfChoice = match hand[0] {
                "A" => ElfChoice::Rock,
                "B" => ElfChoice::Paper,
                "C" => ElfChoice::Scissors,
                _ => panic!("Invalid hand choice!"),
            };

            let my_desired_result = match hand[1] {
                "X" => DesiredResult::Lose(LOST_SCORE),
                "Y" => DesiredResult::Draw(DRAW_SCORE),
                "Z" => DesiredResult::Win(WIN_SCORE),
                _ => panic!("Invalid desired result"),
            };

            return calcluate_score_for_desired_outcome(elf_choice, my_desired_result);
        })
        .sum()

    // Original
    // let mut total_score = 0;
    // for line in input.lines() {
    //     let hand_choices: Vec<&str> = line.split(" ").collect();
    //     let elf_choice: ElfChoice = match hand_choices[0] {
    //         "A" => ElfChoice::Rock,
    //         "B" => ElfChoice::Paper,
    //         "C" => ElfChoice::Scissors,
    //         _ => panic!("Invalid hand choice!"),
    //     };

    //     let my_desired_result = match hand_choices[1] {
    //         "X" => DesiredResult::Lose(LOST_SCORE),
    //         "Y" => DesiredResult::Draw(DRAW_SCORE),
    //         "Z" => DesiredResult::Win(WIN_SCORE),
    //         _ => panic!("Invalid desired result"),
    //     };

    //     let hand_result = calcluate_score_for_desired_outcome(elf_choice, my_desired_result);
    //     total_score += hand_result;
    // }
    // total_score
}

fn calcluate_score_for_desired_outcome(
    elf_choice: ElfChoice,
    desired_result: DesiredResult,
) -> u32 {
    match (elf_choice, desired_result) {
        //
        (ElfChoice::Rock, DesiredResult::Lose(score_worth)) => SCISSOR_VALUE + score_worth,
        (ElfChoice::Rock, DesiredResult::Draw(score_worth)) => ROCK_VALUE + score_worth,
        (ElfChoice::Rock, DesiredResult::Win(score_worth)) => PAPER_VALUE + score_worth,
        //
        (ElfChoice::Paper, DesiredResult::Lose(score_worth)) => ROCK_VALUE + score_worth,
        (ElfChoice::Paper, DesiredResult::Draw(score_worth)) => PAPER_VALUE + score_worth,
        (ElfChoice::Paper, DesiredResult::Win(score_worth)) => SCISSOR_VALUE + score_worth,
        //
        (ElfChoice::Scissors, DesiredResult::Lose(score_worth)) => PAPER_VALUE + score_worth,
        (ElfChoice::Scissors, DesiredResult::Draw(score_worth)) => SCISSOR_VALUE + score_worth,
        (ElfChoice::Scissors, DesiredResult::Win(score_worth)) => ROCK_VALUE + score_worth,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_give_a_winning_score() {
        let input = include_str!("../sample.txt");
        let result = calculate_result(&input);
        assert_eq!(15, result)
    }

    #[test]
    fn it_should_give_the_correct_score() {
        let input = include_str!("../sample.txt");
        let result = calculate_correct_score(&input);
        assert_eq!(12, result)
    }
}
