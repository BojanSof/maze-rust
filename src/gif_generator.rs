use crate::cell::Cell;
use crate::maze::Maze;
use crate::maze_image::{END_COLOR, PATH_COLOR, SOLUTION_PATH_COLOR, START_COLOR, WALL_COLOR};
use gif::{Encoder, Frame, Repeat};
use image::Rgb;
use std::collections::HashSet;
use std::fs::File;

const PALETTE: &[u8] = &[
    WALL_COLOR.0[0],
    WALL_COLOR.0[1],
    WALL_COLOR.0[2], // 0
    PATH_COLOR.0[0],
    PATH_COLOR.0[1],
    PATH_COLOR.0[2], // 1
    START_COLOR.0[0],
    START_COLOR.0[1],
    START_COLOR.0[2], // 2
    END_COLOR.0[0],
    END_COLOR.0[1],
    END_COLOR.0[2], // 3
    SOLUTION_PATH_COLOR.0[0],
    SOLUTION_PATH_COLOR.0[1],
    SOLUTION_PATH_COLOR.0[2], // 4
];

pub fn save_history_to_gif(
    height: usize,
    width: usize,
    start: (usize, usize),
    end: (usize, usize),
    history: &[(usize, usize, Cell)],
    path: &str,
    scale: u32,
    delay: u16,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut maze = Maze {
        cells: vec![vec![Cell::Wall; width]; height],
        start,
        end,
    };

    let mut image_file = File::create(path)?;
    let mut encoder = Encoder::new(
        &mut image_file,
        (width as u32 * scale) as u16,
        (height as u32 * scale) as u16,
        PALETTE,
    )?;
    encoder.set_repeat(Repeat::Finite(0))?;

    let mut frame_buffer = maze_to_indexed_buffer(&maze, scale, &HashSet::new());

    for (y, x, cell) in history {
        maze.cells[*y][*x] = *cell;
        update_frame_buffer(&mut frame_buffer, &maze, *y, *x, scale, &HashSet::new());
        let mut frame = Frame::from_indexed_pixels(
            (width as u32 * scale) as u16,
            (height as u32 * scale) as u16,
            &frame_buffer[..],
            None,
        );
        frame.delay = delay;
        encoder.write_frame(&frame)?;
    }

    Ok(())
}

pub fn save_maze_with_path_to_gif(
    maze: &Maze,
    path_to_solve: &[(usize, usize)],
    file_path: &str,
    scale: u32,
    delay: u16,
) -> Result<(), Box<dyn std::error::Error>> {
    let width = maze.cells[0].len() as u32 * scale;
    let height = maze.cells.len() as u32 * scale;

    let mut image_file = File::create(file_path)?;
    let mut encoder = Encoder::new(&mut image_file, width as u16, height as u16, PALETTE)?;
    encoder.set_repeat(Repeat::Finite(0))?;

    let mut path_set = HashSet::new();
    let mut frame_buffer = maze_to_indexed_buffer(maze, scale, &path_set);

    let mut frame =
        Frame::from_indexed_pixels(width as u16, height as u16, &frame_buffer[..], None);
    frame.delay = delay;
    encoder.write_frame(&frame)?;

    for (y, x) in path_to_solve {
        path_set.insert((*y, *x));
        update_frame_buffer(&mut frame_buffer, maze, *y, *x, scale, &path_set);
        let mut frame =
            Frame::from_indexed_pixels(width as u16, height as u16, &frame_buffer[..], None);
        frame.delay = delay;
        encoder.write_frame(&frame)?;
    }

    Ok(())
}

fn maze_to_indexed_buffer(maze: &Maze, scale: u32, path: &HashSet<(usize, usize)>) -> Vec<u8> {
    let width = maze.cells[0].len() as u32 * scale;
    let height = maze.cells.len() as u32 * scale;
    let mut buffer = vec![0; (width * height) as usize];

    for (y, row) in maze.cells.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            let color_index = get_color_index(maze, y, x, cell, path);
            for i in 0..scale {
                for j in 0..scale {
                    let index = ((y as u32 * scale + i) * width + (x as u32 * scale + j)) as usize;
                    buffer[index] = color_index;
                }
            }
        }
    }
    buffer
}

fn update_frame_buffer(
    frame_buffer: &mut Vec<u8>,
    maze: &Maze,
    y: usize,
    x: usize,
    scale: u32,
    path: &HashSet<(usize, usize)>,
) {
    let width = maze.cells[0].len() as u32 * scale;
    let cell = &maze.cells[y][x];
    let color_index = get_color_index(maze, y, x, cell, path);
    for i in 0..scale {
        for j in 0..scale {
            let index = ((y as u32 * scale + i) * width + (x as u32 * scale + j)) as usize;
            frame_buffer[index] = color_index;
        }
    }
}

fn get_color_index(
    maze: &Maze,
    y: usize,
    x: usize,
    cell: &Cell,
    path: &HashSet<(usize, usize)>,
) -> u8 {
    if (y, x) == maze.start {
        2
    } else if (y, x) == maze.end {
        3
    } else if path.contains(&(y, x)) {
        4
    } else {
        match cell {
            Cell::Wall => 0,
            Cell::Path => 1,
        }
    }
}
