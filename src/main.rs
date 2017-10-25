use std::io;

fn main() {
    let mut grid = vec![
        vec!["_", "_", "x", "_"],
        vec!["x", "_", "x", "_"],
        vec!["_", "_", "x", "_"],
        vec!["_", "_", "_", "_"],
    ];

        print_grid(&grid);


        loop {
            update_grid(&mut grid);
            print_grid(&grid);

            let mut continue_loop = String::new();
            io::stdin().read_line(&mut continue_loop).expect("Please type either Y or N");

            let continue_loop = continue_loop.trim();

            if continue_loop == "Y" {
                continue
            } else {
                break
            }
        }
}

fn update_grid(grid: &mut Vec<Vec<&str>>) -> () {
    let old_grid = grid.clone();

    for y in 0..old_grid.len() {
        for x in 0..old_grid[y].len() {
            let neighbors = neighbor_count(&old_grid, x, y);

            if is_alive(&old_grid, x, y) {
                if neighbors >= 4 {
                    grid[y][x] = "_";
                }

                if neighbors <= 1 {
                    grid[y][x] = "_";
                }

            } else {
                if neighbors == 3 {
                    grid[y][x] = "x";
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
