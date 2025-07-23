mod cell;
mod generators;
mod gif_generator;
mod maze;
mod maze_image;
mod maze_pprint;
mod priority_queue;
mod progress;
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
    let generate_gifs = true;
    let gif_generator_delay = 1; // Delay in milliseconds for generator GIFs
    let gif_solver_delay = 10; // Delay in milliseconds for solver GIFs

    let generators: Vec<(&str, &dyn MazeGenerator)> = vec![
        ("prims", &PrimMazeGenerator),
        ("recursive_backtracker", &RecursiveBacktrackerMazeGenerator),
        ("kruskal", &KruskalMazeGenerator),
        ("aldous_broder", &AldousBroderMazeGenerator),
    ];

    for (generator_name, generator) in generators {
        println!("\n--- Generating maze with {} ---", generator_name);
        let mut tracker = progress::ProgressTracker::new(generate_gifs);
        let gen_start = Instant::now();
        let imperfect_percentage = 20.0; // Percentage of walls to remove for imperfect mazes
        let maze = generator.generate(
            height,
            width,
            Some(start),
            Some(end),
            imperfect_percentage,
            Some(&mut tracker),
        );
        let gen_duration = gen_start.elapsed();
        println!("Maze generation took: {:.2?}", gen_duration);

        if generate_gifs {
            let gif_path = format!("mazes/generated_maze_{}.gif", generator_name);
            println!("Saving generation GIF to {}...", gif_path);
            let gif_start = Instant::now();
            if let Err(e) = gif_generator::save_history_to_gif(
                height,
                width,
                start,
                end,
                &tracker.history,
                &gif_path,
                scale,
                gif_generator_delay,
            ) {
                eprintln!("Error saving generation GIF: {}", e);
            }
            let gif_duration = gif_start.elapsed();
            println!("Saving generation GIF took: {:.2?}", gif_duration);
        }

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
            let mut tracker = progress::ProgressTracker::new(generate_gifs);
            let solve_start = Instant::now();
            match solver.solve(&maze, Some(&mut tracker)) {
                Some(path) => {
                    let solve_duration = solve_start.elapsed();
                    println!("Path found with {}.", solver_name);
                    println!("Solving took: {:.2?}", solve_duration);

                    if generate_gifs {
                        let gif_path = format!(
                            "mazes/generated_maze_{}_solved_{}.gif",
                            generator_name, solver_name
                        );
                        println!("Saving solving GIF to {}...", gif_path);
                        let gif_start = Instant::now();
                        if let Err(e) = gif_generator::save_maze_with_path_to_gif(
                            &maze,
                            &path,
                            &gif_path,
                            scale,
                            gif_solver_delay,
                        ) {
                            eprintln!("Error saving solving GIF: {}", e);
                        }
                        let gif_duration = gif_start.elapsed();
                        println!("Saving solving GIF took: {:.2?}", gif_duration);
                    }

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
