use crate::cell::Cell;
use crate::colors::{END_COLOR, PATH_COLOR, SOLUTION_PATH_COLOR, START_COLOR, WALL_COLOR};
use crate::maze::Maze;
use image::{ImageBuffer, Rgb, RgbImage};
use std::collections::HashSet;

pub fn save_maze_to_image(maze: &Maze, path: &str) -> Result<(), image::ImageError> {
    save_maze_to_image_scaled(maze, path, 1)
}

pub fn save_maze_to_image_scaled(
    maze: &Maze,
    path: &str,
    scale: u32,
) -> Result<(), image::ImageError> {
    let width = maze.cells[0].len() as u32 * scale;
    let height = maze.cells.len() as u32 * scale;
    let mut img: RgbImage = ImageBuffer::new(width, height);

    for (y, row) in maze.cells.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            let color = if (y, x) == maze.start {
                START_COLOR
            } else if (y, x) == maze.end {
                END_COLOR
            } else {
                match cell {
                    Cell::Wall => WALL_COLOR,
                    Cell::Path => PATH_COLOR,
                }
            };
            for i in 0..scale {
                for j in 0..scale {
                    img.put_pixel(x as u32 * scale + j, y as u32 * scale + i, color);
                }
            }
        }
    }
    img.save(path)
}

pub fn save_maze_with_path_to_image(
    maze: &Maze,
    path: &[(usize, usize)],
    file_path: &str,
) -> Result<(), image::ImageError> {
    save_maze_with_path_to_image_scaled(maze, path, file_path, 1)
}

pub fn save_maze_with_path_to_image_scaled(
    maze: &Maze,
    path: &[(usize, usize)],
    file_path: &str,
    scale: u32,
) -> Result<(), image::ImageError> {
    let width = maze.cells[0].len() as u32 * scale;
    let height = maze.cells.len() as u32 * scale;
    let mut img: RgbImage = ImageBuffer::new(width, height);

    let path_set: HashSet<&(usize, usize)> = path.iter().collect();

    for (y, row) in maze.cells.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            let color = if (y, x) == maze.start {
                START_COLOR
            } else if (y, x) == maze.end {
                END_COLOR
            } else if path_set.contains(&(y, x)) {
                SOLUTION_PATH_COLOR
            } else {
                match cell {
                    Cell::Wall => WALL_COLOR,
                    Cell::Path => PATH_COLOR,
                }
            };
            for i in 0..scale {
                for j in 0..scale {
                    img.put_pixel(x as u32 * scale + j, y as u32 * scale + i, color);
                }
            }
        }
    }
    img.save(file_path)
}

pub fn load_maze_from_image(path: &str) -> Result<Maze, String> {
    load_maze_from_image_scaled(path, 1)
}

pub fn load_maze_from_image_scaled(path: &str, scale: u32) -> Result<Maze, String> {
    let img = image::open(path).map_err(|e| e.to_string())?.to_rgb8();
    let (width, height) = img.dimensions();

    if width % scale != 0 || height % scale != 0 {
        return Err("Image dimensions are not divisible by the scale factor.".to_string());
    }

    let maze_width = (width / scale) as usize;
    let maze_height = (height / scale) as usize;
    let mut cells = vec![vec![Cell::Path; maze_width]; maze_height];
    let mut start = None;
    let mut end = None;

    for y in 0..maze_height {
        for x in 0..maze_width {
            let pixel = img.get_pixel(x as u32 * scale, y as u32 * scale);
            match *pixel {
                START_COLOR => {
                    if start.is_some() {
                        return Err("Multiple start points found".to_string());
                    }
                    start = Some((y, x));
                    cells[y][x] = Cell::Path;
                }
                END_COLOR => {
                    if end.is_some() {
                        return Err("Multiple end points found".to_string());
                    }
                    end = Some((y, x));
                    cells[y][x] = Cell::Path;
                }
                WALL_COLOR => {
                    cells[y][x] = Cell::Wall;
                }
                PATH_COLOR => {
                    cells[y][x] = Cell::Path;
                }
                _ => {
                    cells[y][x] = Cell::Path;
                }
            }
        }
    }

    let start = start.ok_or_else(|| "No start point found".to_string())?;
    let end = end.ok_or_else(|| "No end point found".to_string())?;

    Ok(Maze { cells, start, end })
}
