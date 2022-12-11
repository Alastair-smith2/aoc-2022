fn determine_whether_to_check_tree_is_visible(
    row_idx: usize,
    column_idx: usize,
    row_width: usize,
    column_height: usize,
) -> bool {
    !(row_idx == 0
        || column_idx == 0
        || row_idx == (column_height - 1)
        || column_idx == (row_width - 1))
}

fn check_whether_tree_is_visible(
    grid: &Vec<Vec<u32>>,
    row_idx: usize,
    column_idx: usize,
    row_width: usize,
    column_height: usize,
    tree_height: u32,
) -> bool {
    let was_tree_visible = check_adjacent_row_values(
        grid,
        row_idx,
        column_idx,
        row_width,
        column_height,
        tree_height,
    ) || check_adjacent_column_values(
        grid,
        column_idx,
        row_idx,
        row_width,
        column_height,
        tree_height,
    );
    was_tree_visible
}

fn check_adjacent_row_values(
    grid: &Vec<Vec<u32>>,
    row_idx: usize,
    column_idx: usize,
    row_width: usize,
    column_height: usize,
    tree_value: u32,
) -> bool {
    let mut visible_from_left = true;
    let mut visible_from_right = true;
    let range = 0..column_height;
    let range_backwards = 0..column_height;
    for column in range {
        if column >= column_idx {
            break;
        }
        let grid_val = grid[row_idx][column];
        if grid_val >= tree_value {
            visible_from_left = false;
            break;
        }
    }

    if !visible_from_left {
        let range_backward = range_backwards.rev();
        for column in range_backward {
            if column <= column_idx {
                break;
            }
            let grid_val = grid[row_idx][column];
            if grid_val >= tree_value {
                visible_from_right = false;
                break;
            }
        }
    }
    let visible = visible_from_left || visible_from_right;
    visible
}

fn check_adjacent_column_values(
    grid: &Vec<Vec<u32>>,
    column_idx: usize,
    row_idx: usize,
    row_width: usize,
    column_height: usize,
    tree_value: u32,
) -> bool {
    let mut visible_from_top = true;
    let mut visible_from_bottom = true;
    let range = 0..row_width;
    let range_backwards = 0..row_width;
    for row in range {
        if row >= row_idx {
            break;
        }

        let grid_val = grid[row][column_idx];
        if grid_val >= tree_value {
            visible_from_top = false;
            break;
        }
    }

    if !visible_from_top {
        for row in range_backwards.rev() {
            if row <= row_idx {
                break;
            }
            let grid_val = grid[row][column_idx];
            if grid_val >= tree_value {
                visible_from_bottom = false;
                break;
            }
        }
    }

    let visible = visible_from_top || visible_from_bottom;
    visible
}

pub fn get_total_trees_visible(input: &str) -> usize {
    let grid_rows = input.lines().collect::<Vec<_>>();

    let grid_width_size = *&grid_rows[0].len();

    let grid_height_size = grid_rows.len();

    let mut grid = vec![vec![0; grid_width_size]; grid_height_size];

    for (l_idx, l) in grid_rows.iter().enumerate() {
        for (c_idx, c) in l.chars().enumerate() {
            grid[l_idx][c_idx] = c.to_digit(10).unwrap();
        }
    }

    let mut tree_visible = 0;

    for row in 0..grid_width_size {
        for column in 0..grid_height_size {
            let tree_value = grid[row][column];
            if determine_whether_to_check_tree_is_visible(
                row,
                column,
                grid_width_size,
                grid_height_size,
            ) {
                if check_whether_tree_is_visible(
                    &grid,
                    row,
                    column,
                    grid_width_size,
                    grid_height_size,
                    tree_value,
                ) {
                    println!("This tree value is visible? {tree_value}");
                    tree_visible += 1;
                }
            }
        }
    }
    tree_visible += (grid_width_size - 1) * 2;
    tree_visible += (grid_width_size - 1) * 2;

    tree_visible
}
