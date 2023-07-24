use rand::prelude::*;
use std::fmt::Display;
use structopt::StructOpt;

struct Maze {
    width: usize,
    height: usize,
    grid: Vec<Vec<bool>>,
}

impl Maze {
    fn new(mut width: usize, mut height: usize) -> Self {
        // Make dimensions odd
        if width % 2 == 0 {
            width = width + 1;
        }
        if height % 2 == 0 {
            height += 1;
        }

        // Initialize maze
        let mut grid: Vec<Vec<bool>> = vec![];
        for i in 0..height {
            grid.push(vec![]);
            for j in 0..width {
                grid[i].push(!(i % 2 == 1 && j % 2 == 1))
            }
        }

        // Carve maze paths
        for col in (1..width - 1).step_by(2) {
            for row in (1..height - 1).step_by(2) {
                let south: bool = if row == (height - 2) {
                    false
                } else if col == (width - 2) {
                    true
                } else {
                    random()
                };
                if (col == width - 2) && (row == height - 2) {
                    break;
                }
                if south {
                    grid[row + 1][col] = false;
                } else {
                    grid[row][col + 1] = false;
                }
            }
        }

        // Carve maze entry and exit
        grid[0][1] = false;
        grid[height - 1][width - 2] = false;

        Maze {
            width,
            height,
            grid,
        }
    }
}

impl Display for Maze {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.height {
            for j in 0..self.width {
                if self.grid[i][j] {
                    write!(f, "█")?;
                } else {
                    write!(f, " ")?;
                }
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}
#[derive(Debug, StructOpt)]
#[structopt(name = "example", about = "An example of StructOpt usage.")]
struct Opt {
    #[structopt(short, long, default_value = "39")]
    width: usize,
    #[structopt(short, long, default_value = "19")]
    height: usize,
}

fn main() {
    let opt = Opt::from_args();
    let maze = Maze::new(opt.width, opt.height);

    println!("{}", &maze);
}
