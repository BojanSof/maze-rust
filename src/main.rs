mod cell;
mod dfs_solver;
mod maze;
mod maze_pprint;
mod solver;
mod stack;

use crate::solver::Solver;

fn main() {
    let maze_path = "mazes/maze3.txt";
    match maze::Maze::from_file(maze_path) {
        Ok(maze) => {
            println!("Maze loaded successfully!");
            println!("{}", maze);
            let solver = dfs_solver::DfsSolver;
            match solver.solve(&maze) {
                Some(path) => {
                    println!("Path found:");
                    for (row, col) in &path {
                        println!("({}, {})", row, col);
                    }
                    let maze_with_path = maze_pprint::display_maze_with_path(&maze, &path);
                    println!("Maze with path:\n{}", maze_with_path);
                }
                None => {
                    println!("No path found in the maze.");
                }
            }
        }
        Err(e) => {
            eprintln!("Error loading maze: {:?}", e);
        }
    }
}
