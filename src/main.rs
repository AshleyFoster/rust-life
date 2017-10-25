fn main() {
    let grid = vec![
        vec!["_", "_", "x", "_"],
        vec!["x", "x", "x", "_"],
        vec!["_", "_", "x", "_"],
        vec!["_", "_", "_", "_"],
    ];

        print_grid(&grid);

        let mut new_grid = grid.clone();
        update_grid(&grid, &mut new_grid);
        print_grid(&new_grid);
}

fn update_grid(grid: &Vec<Vec<&str>>, new_grid: &mut Vec<Vec<&str>>) -> () {

    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            let neighbors = neighbor_count(&grid, x, y);

            if is_alive(&grid, x, y) {
                if neighbors >= 4 {
                    new_grid[y][x] = "_";
                }

                if neighbors <= 1 {
                    new_grid[y][x] = "_";
                }

            } else {
                if neighbors == 3 {
                    new_grid[y][x] = "x";
                }
            }
        }
    }
}

fn neighbor_count(grid: &Vec<Vec<&str>>, current_x: usize, current_y: usize) -> usize {
    let mut count = 0;
    if current_x > 0 && current_y > 0 {
        if is_alive(grid, current_x - 1, current_y - 1) {
            count += 1;
        }
    }

    if current_y > 0 {
        if is_alive(grid, current_x, current_y - 1) {
            count += 1;
        }

        if is_alive(grid, current_x + 1, current_y - 1) {
            count += 1;
        }
    }

    if current_x > 0 {
        if is_alive(grid, current_x - 1, current_y) {
            count += 1;
        }

        if is_alive(grid, current_x - 1, current_y + 1) {
            count += 1;
        }
    }

    if is_alive(grid, current_x, current_y + 1) {
        count += 1;
    }

    if is_alive(grid, current_x + 1, current_y + 1) {
        count += 1;
    }

    if is_alive(grid, current_x + 1, current_y) {
        count += 1;
    }

    count
}

fn is_alive(grid: &Vec<Vec<&str>>, x: usize, y: usize) -> bool {
    match grid.get(y) {
        None => false,
        Some(row) => {
            match row.get(x) {
                None => false,
                Some(cell) => {
                    *cell == "x"
                }
            }
        }
    }
}

fn print_grid(grid: &Vec<Vec<&str>>) -> () {
    for row in grid {
        for cell in row.iter() {
            print!("{}", cell);
        }
        println!("");
    }
}
