use rand::Rng;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * std::f64::consts::PI / 180.0
}

pub fn random_double() -> f64 {
    rand::thread_rng().gen()
}

pub fn random_double_range(min: f64, max: f64) -> f64 {
    min + (max - min) * random_double()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random_double_range() {
        for _ in 0..1000 {
            let r = random_double_range(5.0, 10.0);
            assert!(r >= 5.0 && r < 10.0);
        }
    }

    #[test]
    fn test_degrees_to_radians() {
        assert!((degrees_to_radians(180.0) - std::f64::consts::PI).abs() < 1e-10);
        assert!((degrees_to_radians(360.0) - 2.0 * std::f64::consts::PI).abs() < 1e-10);
    }
}
