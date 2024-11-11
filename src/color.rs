use std::io::Write;
use crate::vec3::Color;

pub fn write_color(out: &mut dyn Write, pixel_color: Color) -> std::io::Result<()> {
    // Convert the [0,1] color components to [0,255]
    let r = (255.999 * pixel_color.x()) as i32;
    let g = (255.999 * pixel_color.y()) as i32;
    let b = (255.999 * pixel_color.z()) as i32;

    // Write the color values
    writeln!(out, "{} {} {}", r, g, b)
}
