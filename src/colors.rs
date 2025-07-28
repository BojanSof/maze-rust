//! Color definitions (based on Matplotlib 'tab10' palette)
use image::Rgb;

/// Wall (background) color: gray
pub const WALL_COLOR: Rgb<u8> = Rgb([0x7f, 0x7f, 0x7f]);
/// Open path (corridor) color: white
pub const PATH_COLOR: Rgb<u8> = Rgb([0xff, 0xff, 0xff]);
/// Start cell color: green
pub const START_COLOR: Rgb<u8> = Rgb([0x2c, 0xa0, 0x2c]);
/// End cell color: red
pub const END_COLOR: Rgb<u8> = Rgb([0xd6, 0x27, 0x28]);
/// Solution path color: blue
pub const SOLUTION_PATH_COLOR: Rgb<u8> = Rgb([0x1f, 0x77, 0xb4]);
/// Visited cell overlay color: orange
pub const VISITED_COLOR: Rgb<u8> = Rgb([0xff, 0x7f, 0x0e]);
/// Current cell highlight color: purple
pub const CURRENT_COLOR: Rgb<u8> = Rgb([0x94, 0x67, 0xbd]);
