use crate::interval::Interval;
use crate::vec3::Color;
use std::io::Write;

pub fn write_color(
    out: &mut dyn Write,
    pixel_color: Color,
    samples_per_pixel: i32,
) -> std::io::Result<()> {
    let scale = 1.0 / samples_per_pixel as f64;
    let r = pixel_color.x() * scale;
    let g = pixel_color.y() * scale;
    let b = pixel_color.z() * scale;

    let intensity = Interval::new(0.0, 0.999);

    writeln!(
        out,
        "{} {} {}",
        (256.0 * intensity.clamp(r)) as i32,
        (256.0 * intensity.clamp(g)) as i32,
        (256.0 * intensity.clamp(b)) as i32
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_write_color_with_samples() {
        let mut buffer = Cursor::new(Vec::new());
        let color = Color::new(2.0, 4.0, 0.0);
        write_color(&mut buffer, color, 2).unwrap();

        let output = String::from_utf8(buffer.into_inner()).unwrap();
        assert_eq!(output.trim(), "255 255 0");
    }
}
