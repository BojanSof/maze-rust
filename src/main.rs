mod astar;
mod bfs_solver;
mod cell;
mod dfs_solver;
mod dijkstra;
mod maze;
mod maze_pprint;
mod priority_queue;
mod queue;
mod solver;
mod stack;

use crate::solver::Solver;

fn main() {
    let maze_path = "mazes/maze3.txt";
    match maze::Maze::from_file(maze_path) {
        Ok(maze) => {
            println!("Maze loaded successfully!");
            println!("{}", maze);
            println!("Maze as graph: \n{:?}", maze.to_graph());
            //let solver = dfs_solver::DfsSolver;
            //let solver = bfs_solver::BfsSolver;
            //let solver = dijkstra::DijkstraSolver;
            let solver = astar::AstarSolver;
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
