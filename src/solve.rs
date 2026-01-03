use std::collections::VecDeque;

pub fn count_islands(input: String) -> u8 {
    let mut grid: Vec<bool> = input
        .chars()
        .filter_map(|c| match c {
            '\n' => None,
            '1' => Some(true),
            '0' => Some(false),
            _ => None,
        })
        .collect();

    let rows = input.lines().count();
    let cols = input.lines().next().unwrap().chars().count();
    solve_count(&mut grid, rows, cols)
}

fn flat_index(row: usize, col: usize, cols: usize) -> usize {
    row * cols + col
}

fn index(total: usize, cols: usize) -> (usize, usize) {
    let c = total % cols;
    let r = total / cols;
    (r, c)
}

fn solve_count(grid: &mut [bool], rows: usize, cols: usize) -> u8 {
    let mut count = 0;

    for n in 0..cols * rows {
        if grid[n] {
            clear_island(grid, rows, cols, n);
            count += 1;
        }
    }

    count
}

fn clear_island(grid: &mut [bool], rows: usize, cols: usize, pos: usize) {
    let mut queue = VecDeque::new();
    queue.push_back(pos);

    while let Some(cell) = queue.pop_front() {
        let (r, c) = index(cell, cols);

        let directions: [(isize, isize); 4] = [(0, 1), (1, 0), (-1, 0), (0, -1)];

        for direction in directions {
            let cr = r as isize + direction.0;
            let cc = c as isize + direction.1;

            if cr >= 0 && cr < rows as isize && cc >= 0 && cc < cols as isize {
                let index = flat_index(cr as usize, cc as usize, cols);
                if grid[index] {
                    queue.push_back(index);
                }
            }
        }

        grid[cell] = false; //delete as we go
    }
}
