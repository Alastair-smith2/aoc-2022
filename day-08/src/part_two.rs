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

pub fn get_tree_scenic_value(
    grid: &Vec<Vec<u32>>,
    row_idx: usize,
    column_idx: usize,
    row_width: usize,
    column_height: usize,
    tree_height: u32,
) -> usize {
    let adjacent_row_view =
        check_adjacent_row_values(grid, row_idx, column_idx, column_height, tree_height);
    let adjacent_column_view =
        check_adjacent_column_values(grid, column_idx, row_idx, row_width, tree_height);

    adjacent_row_view * adjacent_column_view
}

fn check_adjacent_row_values(
    grid: &Vec<Vec<u32>>,
    row_idx: usize,
    column_idx: usize,
    column_height: usize,
    tree_value: u32,
) -> usize {
    let mut visibility_to_left = 0;
    let mut visibility_to_right = 0;
    let range = 0..column_idx;
    let range_backwards = column_idx + 1..column_height;

    for (idx, column) in range.rev().enumerate() {
        let grid_val = grid[row_idx][column];
        if grid_val >= tree_value {
            visibility_to_left += 1;
            break;
        } else {
            visibility_to_left = idx + 1;
        }
    }

    for (idx, column) in range_backwards.enumerate() {
        let grid_val = grid[row_idx][column];
        if grid_val >= tree_value {
            visibility_to_right += 1;
            break;
        } else {
            visibility_to_right = idx + 1;
        }
    }

    let visible_for_rows = visibility_to_left * visibility_to_right;
    visible_for_rows
}

fn check_adjacent_column_values(
    grid: &Vec<Vec<u32>>,
    column_idx: usize,
    row_idx: usize,
    row_width: usize,
    tree_value: u32,
) -> usize {
    let mut visible_from_top = 0;
    let mut visible_from_bottom = 0;
    let range = 0..row_idx;
    let range_backwards = row_idx + 1..row_width;
    for (idx, row) in range.rev().enumerate() {
        let grid_val = grid[row][column_idx];
        if grid_val >= tree_value {
            visible_from_top += 1;
            break;
        } else {
            visible_from_top = idx + 1;
        }
    }

    for (idx, row) in range_backwards.enumerate() {
        let grid_val = grid[row][column_idx];
        if grid_val >= tree_value {
            visible_from_bottom += 1;
            break;
        } else {
            visible_from_bottom = idx + 1;
        }
    }

    let visible_for_columns = visible_from_top * visible_from_bottom;
    visible_for_columns
}

pub fn check_most_scenic_tree(input: &str) -> usize {
    let grid_rows = input.lines().collect::<Vec<_>>();

    let grid_width_size = *&grid_rows[0].len();

    let grid_height_size = grid_rows.len();

    let mut grid = vec![vec![0; grid_width_size]; grid_height_size];

    for (l_idx, l) in grid_rows.iter().enumerate() {
        for (c_idx, c) in l.chars().enumerate() {
            grid[l_idx][c_idx] = c.to_digit(10).unwrap();
        }
    }

    let mut most_scenic_value = 0;

    for row in 0..grid_width_size {
        for column in 0..grid_height_size {
            let tree_value = grid[row][column];
            if determine_whether_to_check_tree_is_visible(
                row,
                column,
                grid_width_size,
                grid_height_size,
            ) {
                let value = get_tree_scenic_value(
                    &grid,
                    row,
                    column,
                    grid_width_size,
                    grid_height_size,
                    tree_value,
                );
                if value > most_scenic_value {
                    most_scenic_value = value
                }
            }
        }
    }

    // tree_visible
    most_scenic_value
}
