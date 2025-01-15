use crate::vec3::{Point3, Vec3};

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    orig: Point3,
    dir: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Ray {
        Ray {
            orig: origin,
            dir: direction,
        }
    }

    pub fn origin(&self) -> Point3 {
        self.orig
    }

    pub fn direction(&self) -> Point3 {
        self.dir
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.orig + self.dir * t
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ray_creation() {
        let origin = Point3::new(0.0, 0.0, 0.0);
        let direction = Vec3::new(1.0, 0.0, 0.0);
        let ray = Ray::new(origin, direction);

        assert_eq!(ray.origin().x(), 0.0);
        assert_eq!(ray.direction().x(), 1.0);
    }

    #[test]
    fn test_ray_at() {
        let origin = Point3::new(2.0, 3.0, 4.0);
        let direction = Vec3::new(1.0, 0.0, 0.0);
        let ray = Ray::new(origin, direction);

        let point = ray.at(2.0);
        assert_eq!(point.x(), 4.0); // 2.0 + 2.0
        assert_eq!(point.y(), 3.0);
        assert_eq!(point.z(), 4.0);
    }
}
