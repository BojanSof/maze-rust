mod cell;
mod colors;
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
use clap::Parser;
use std::time::Instant;

/// Maze CLI arguments
#[derive(Debug, Parser)]
#[command(author, version, about)]
struct Args {
    /// Maze width
    #[arg(long, default_value_t = 51)]
    width: usize,

    /// Maze height
    #[arg(long, default_value_t = 51)]
    height: usize,

    /// Start row (0-based)
    #[arg(long)]
    start_row: Option<usize>,

    /// Start column (0-based)
    #[arg(long)]
    start_col: Option<usize>,

    /// End row (0-based)
    #[arg(long)]
    end_row: Option<usize>,

    /// End column (0-based)
    #[arg(long)]
    end_col: Option<usize>,

    /// Path scale factor for PNG/GIF output
    #[arg(long, default_value_t = 10)]
    scale: u32,

    /// Enable GIF generation
    #[arg(long, default_value_t = true)]
    generate_gifs: bool,

    /// Delay in ms for generator GIF frames
    #[arg(long, default_value_t = 1)]
    gif_generator_delay: u16,

    /// Delay in ms for solver GIF frames
    #[arg(long, default_value_t = 10)]
    gif_solver_delay: u16,

    /// Output directory for images/GIFs
    #[arg(long, default_value = "mazes")]
    output_dir: String,

    /// Percentage of walls to remove for imperfect (random) mazes
    #[arg(long, default_value_t = 20.0)]
    imperfect_percentage: f32,

    /// Maze generator algorithms to run (comma-delimited)
    #[arg(
        long,
        value_delimiter = ',',
        default_value = "prims,recursive_backtracker,kruskal,aldous_broder"
    )]
    generators: Vec<String>,

    /// Solver algorithms to run (comma-delimited)
    #[arg(long, value_delimiter = ',', default_value = "astar,bfs,dfs,dijkstra")]
    solvers: Vec<String>,
}

fn main() {
    let args = Args::parse();
    let width = args.width;
    let height = args.height;
    let start = (args.start_row.unwrap_or(1), args.start_col.unwrap_or(1));
    let end = (
        args.end_row.unwrap_or(height - 2),
        args.end_col.unwrap_or(width - 2),
    );
    let scale = args.scale;
    let generate_gifs = args.generate_gifs;
    let gif_generator_delay = args.gif_generator_delay;
    let gif_solver_delay = args.gif_solver_delay;
    let output_dir = args.output_dir;
    // Ensure output directory exists
    if let Err(e) = std::fs::create_dir_all(&output_dir) {
        eprintln!("Failed to create output directory '{}': {}", output_dir, e);
        std::process::exit(1);
    }

    // Resolve generator implementations from names
    let available_generators: &[(&str, &dyn MazeGenerator)] = &[
        ("prims", &PrimMazeGenerator),
        ("recursive_backtracker", &RecursiveBacktrackerMazeGenerator),
        ("kruskal", &KruskalMazeGenerator),
        ("aldous_broder", &AldousBroderMazeGenerator),
    ];
    let generators: Vec<(&str, &dyn MazeGenerator)> = args
        .generators
        .iter()
        .filter_map(|name| {
            available_generators
                .iter()
                .find(|(n, _)| n == name)
                .copied()
        })
        .collect();

    for (generator_name, generator) in generators {
        println!("\n--- Generating maze with {} ---", generator_name);
        let mut tracker = progress::ProgressTracker::new(generate_gifs);
        let gen_start = Instant::now();
        let imperfect_percentage = args.imperfect_percentage;
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
            let gif_path = format!("{}/generated_maze_{}.gif", output_dir, generator_name);
            println!("Saving generation GIF to {}...", gif_path);
            let gif_start = Instant::now();
            if let Err(e) = gif_generator::save_history_to_gif(
                &maze,
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

        let generated_maze_path = format!("{}/generated_maze_{}.png", output_dir, generator_name);
        let save_start = Instant::now();
        if let Err(e) = maze_image::save_maze_to_image_scaled(&maze, &generated_maze_path, scale) {
            eprintln!("Error saving generated maze: {}", e);
            continue;
        }
        let save_duration = save_start.elapsed();
        println!("Saving generated maze took: {:.2?}", save_duration);

        // Resolve solver implementations from names
        let available_solvers: &[(&str, &dyn Solver)] = &[
            ("astar", &solvers::astar::AstarSolver),
            ("bfs", &solvers::bfs_solver::BfsSolver),
            ("dfs", &solvers::dfs_solver::DfsSolver),
            ("dijkstra", &solvers::dijkstra::DijkstraSolver),
        ];
        let solvers: Vec<(&str, &dyn Solver)> = args
            .solvers
            .iter()
            .filter_map(|name| available_solvers.iter().find(|(n, _)| n == name).copied())
            .collect();

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
                            "{}/generated_maze_{}_solved_{}.gif",
                            output_dir, generator_name, solver_name
                        );
                        println!("Saving solving GIF to {}...", gif_path);
                        let gif_start = Instant::now();
                        if let Err(e) = gif_generator::save_solver_history_to_gif(
                            &maze,
                            &tracker.history,
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
                        "{}/generated_maze_{}_solved_{}.png",
                        output_dir, generator_name, solver_name
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
