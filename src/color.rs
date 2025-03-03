use crate::interval::Interval;
use crate::vec3::Color;
use std::io::Write;

pub fn write_color(
    out: &mut dyn Write,
    pixel_color: Color,
    samples_per_pixel: i32,
) -> std::io::Result<()> {
    let scale = 1.0 / samples_per_pixel as f64;

    let r = linear_to_gamma(pixel_color.x() * scale);
    let g = linear_to_gamma(pixel_color.y() * scale);
    let b = linear_to_gamma(pixel_color.z() * scale);

    let intensity = Interval::new(0.0, 0.999);

    writeln!(
        out,
        "{} {} {}",
        (256.0 * intensity.clamp(r)) as i32,
        (256.0 * intensity.clamp(g)) as i32,
        (256.0 * intensity.clamp(b)) as i32
    )
}

pub fn color_to_rgb(pixel_color: Color, samples_per_pixel: i32) -> (u8, u8, u8) {
    let scale = 1.0 / samples_per_pixel as f64;

    let r = linear_to_gamma(pixel_color.x() * scale);
    let g = linear_to_gamma(pixel_color.y() * scale);
    let b = linear_to_gamma(pixel_color.z() * scale);

    let intensity = Interval::new(0.0, 0.999);

    (
        (256.0 * intensity.clamp(r)) as u8,
        (256.0 * intensity.clamp(g)) as u8,
        (256.0 * intensity.clamp(b)) as u8,
    )
}

#[cfg(feature = "image")]
pub fn color_to_rgba(pixel_color: Color, samples_per_pixel: i32) -> [u8; 4] {
    let (r, g, b) = color_to_rgb(pixel_color, samples_per_pixel);
    [r, g, b, 255]
}

fn linear_to_gamma(linear_component: f64) -> f64 {
    linear_component.sqrt()
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
    
    #[test]
    fn test_color_to_rgb() {
        let color = Color::new(2.0, 4.0, 0.0);
        let (r, g, b) = color_to_rgb(color, 2);
        assert_eq!(r, 255);
        assert_eq!(g, 255);
        assert_eq!(b, 0);
    }
}
