use std::io;
use std::fmt;

#[derive(Clone)]
struct Cell {
    alive: bool,
}

impl Cell {
    fn new(alive: bool) -> Cell {
        Cell { alive: alive }
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.alive == true {
            write!(f, "x")
        } else {
            write!(f, "_")
        }
    }
}

fn main() {
    let mut grid = vec![
        vec![Cell::new(false), Cell::new(false), Cell::new(true), Cell::new(false)],
        vec![Cell::new(true), Cell::new(false), Cell::new(true), Cell::new(false)],
        vec![Cell::new(false), Cell::new(false), Cell::new(true), Cell::new(false)],
        vec![Cell::new(false), Cell::new(false) , Cell::new(false) , Cell::new(false)],
    ];

        print_grid(&grid);


        loop {
            let mut continue_loop = String::new();
            println!("Would you like to continue? y/n: ");
            io::stdin().read_line(&mut continue_loop).expect("Could not get user input");

            if continue_loop.trim() == "y" {
                update_grid(&mut grid);
                print_grid(&grid);
            } else {
                break
            }
        }
}

fn update_grid(grid: &mut Vec<Vec<Cell>>) -> () {
    let old_grid = grid.clone();

    for y in 0..old_grid.len() {
        for x in 0..old_grid[y].len() {
            let neighbors = neighbor_count(&old_grid, x, y);

            if is_alive(&old_grid, x, y) {
                if neighbors >= 4 {
                    grid[y][x] = Cell::new(false);
                }

                if neighbors <= 1 {
                    grid[y][x] = Cell::new(false);
                }

            } else {
                if neighbors == 3 {
                    grid[y][x] = Cell::new(true);
                }
            }
        }
    }
}

fn neighbor_count(grid: &Vec<Vec<Cell>>, current_x: usize, current_y: usize) -> usize {
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

fn is_alive(grid: &Vec<Vec<Cell>>, x: usize, y: usize) -> bool {
    match grid.get(y) {
        None => false,
        Some(row) => {
            match row.get(x) {
                None => false,
                Some(cell) => cell.alive
            }
        }
    }
}

fn print_grid(grid: &Vec<Vec<Cell>>) -> () {
    for row in grid {
        for cell in row.iter() {
            print!("{}", cell);
        }
        println!("");
    }
}
