mod cell;
mod generators;
mod maze;
mod maze_image;
mod maze_pprint;
mod priority_queue;
mod queue;
mod solvers;
mod stack;

use crate::generators::aldous_broder::AldousBroderMazeGenerator;
use crate::generators::generator::MazeGenerator;
use crate::generators::kruskal::KruskalMazeGenerator;
use crate::generators::prims::PrimMazeGenerator;
use crate::generators::recursive_backtracker::RecursiveBacktrackerMazeGenerator;
use crate::solvers::solver::Solver;
use std::time::Instant;

fn main() {
    let width = 51;
    let height = 51;
    let start = (1, 1);
    let end = (height - 2, width - 2);
    let scale = 10;

    let generators: Vec<(&str, &dyn MazeGenerator)> = vec![
        ("prims", &PrimMazeGenerator),
        ("recursive_backtracker", &RecursiveBacktrackerMazeGenerator),
        ("kruskal", &KruskalMazeGenerator),
        ("aldous_broder", &AldousBroderMazeGenerator),
    ];

    for (generator_name, generator) in generators {
        println!("\n--- Generating maze with {} ---", generator_name);
        let gen_start = Instant::now();
        let imperfect_percentage = 20.0; // Percentage of walls to remove for imperfect mazes
        let maze = generator.generate(height, width, Some(start), Some(end), imperfect_percentage);
        let gen_duration = gen_start.elapsed();
        println!("Maze generation took: {:.2?}", gen_duration);

        let generated_maze_path = format!("mazes/generated_maze_{}.png", generator_name);
        let save_start = Instant::now();
        if let Err(e) = maze_image::save_maze_to_image_scaled(&maze, &generated_maze_path, scale) {
            eprintln!("Error saving generated maze: {}", e);
            continue;
        }
        let save_duration = save_start.elapsed();
        println!("Saving generated maze took: {:.2?}", save_duration);

        let solvers: Vec<(&str, &dyn Solver)> = vec![
            ("astar", &solvers::astar::AstarSolver),
            ("bfs", &solvers::bfs_solver::BfsSolver),
            ("dfs", &solvers::dfs_solver::DfsSolver),
            ("dijkstra", &solvers::dijkstra::DijkstraSolver),
        ];

        for (solver_name, solver) in &solvers {
            println!("\n--- Solving with {} ---", solver_name);
            let solve_start = Instant::now();
            match solver.solve(&maze) {
                Some(path) => {
                    let solve_duration = solve_start.elapsed();
                    println!("Path found with {}.", solver_name);
                    println!("Solving took: {:.2?}", solve_duration);

                    let solved_image_path = format!(
                        "mazes/generated_maze_{}_solved_{}.png",
                        generator_name, solver_name
                    );
                    let save_path_start = Instant::now();
                    if let Err(e) = maze_image::save_maze_with_path_to_image_scaled(
                        &maze,
                        &path,
                        &solved_image_path,
                        scale,
                    ) {
                        eprintln!("Error saving solved maze: {}", e);
                    } else {
                        let save_path_duration = save_path_start.elapsed();
                        println!("Saving solved maze took: {:.2?}", save_path_duration);
                    }
                }
                None => {
                    let solve_duration = solve_start.elapsed();
                    println!("No path found with {}.", solver_name);
                    println!("Solving took: {:.2?}", solve_duration);
                }
            }
        }
    }
}
