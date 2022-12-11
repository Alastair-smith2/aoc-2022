mod parsing;
mod stack;

use parsing::*;
use stack::FileEntry;
const TOTAL_DISK: usize = 70000000;
const UNUSED_SPACE_REQUIRED: usize = 30000000;

fn main() {
    let input = include_str!("../input.txt");
    let part_one = sum_of_small_directories(input);
    println!("The solution to part one {part_one}");
    let part_two = get_space_required_to_be_freed(input);
    println!("The solution to part one {part_two}");
}

// Puzzle input = filesystem trying to find space
// tree of files (plain data) and directories (which can contain other directories or files)
/*
Outer most directory = /
Lines beginning with $ are commands you executed
cd = change directory
    x = move in one level (to directory x)
    .. = move out one level
    / = switch to the most outer directory

ls = list
    123 abc means current directory contain a file named abc with size 123
    dir xyz means current directory contains a directory namedd xyz


Determine total size of each directory...
size of each is sum of current files + nested directories

For part one, find all directories greater than 100000 and sum them (e.g. )
 */

/*

match cd until next cd
Then figure out whether cd has alphanumeric to mean move down or .. to move out
Can parse each line
 */

fn parse_root(input: &str) -> FileEntry {
    let lines = input
        .lines()
        .map(|l| parse_file_line(l))
        .collect::<Vec<_>>();

    let mut stack = vec![FileEntry {
        path: "/".into(),
        size: 0,
        children: vec![],
    }];

    for line in lines {
        println!("{line:?}");
        match line {
            Line::Command(cmd) => match cmd {
                Command::Ls(_) => {
                    // no need to do anything here
                }
                Command::Cd(path) => match path.0 {
                    "/" => {
                        // we start here...
                    }
                    ".." => {
                        let child = stack.pop();
                        stack.last_mut().unwrap().children.push(child.unwrap());
                    }
                    _ => {
                        let node = FileEntry {
                            path: path.0,
                            size: 0,
                            children: vec![],
                        };
                        stack.push(node);
                    }
                },
            },
            Line::Entry(entry) => match entry {
                Entry::Dir(_) => {
                    // no need for an action
                }
                Entry::File(File { size, name }) => {
                    let node = FileEntry {
                        size,
                        path: name,
                        children: vec![],
                    };
                    stack.last_mut().unwrap().children.push(node);
                }
            },
        }
    }

    dbg!(&stack);

    let mut root = stack.pop().unwrap();

    while let Some(mut next) = stack.pop() {
        next.children.push(root);
        root = next;
    }
    root
}

fn sum_of_small_directories(input: &str) -> usize {
    let root = parse_root(input);

    let sum = root
        .all_dirs()
        .map(|d| d.total_size())
        .filter(|&s| s <= 100_000)
        .sum::<usize>();
    sum
}

fn get_space_required_to_be_freed(input: &str) -> usize {
    let root = parse_root(input);

    let used_space = root.total_size();

    let free_space = TOTAL_DISK.checked_sub(dbg!(used_space)).unwrap();

    let min_space_to_free = UNUSED_SPACE_REQUIRED.checked_sub(dbg!(free_space)).unwrap();

    let directory_to_remove = root
        .all_dirs()
        .map(|d| d.total_size())
        .filter(|&s| s >= min_space_to_free)
        .min();

    directory_to_remove.unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one() {
        let input = include_str!("../sample.txt");
        let result = sum_of_small_directories(input);
        assert_eq!(95437, result);
    }

    #[test]
    fn part_two() {
        let input = include_str!("../sample.txt");
        let result = get_space_required_to_be_freed(input);
        assert_eq!(24933642, result);
    }
}
