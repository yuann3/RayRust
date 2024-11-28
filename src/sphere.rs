use crate::vec3::Point3;
use crate::ray::Ray;

pub fn hit_sphere(center: Point3, radius: f64, ray: &Ray) -> bool {
    let oc = ray.origin() - center;
    let a = ray.direction().dot(&ray.direction());
    let half_b = oc.dot(&ray.direction());
    let c = oc.dot(&oc) - radius * radius;
    let discriminant = half_b * half_b - a * c;

    if discriminant < 0.0 {
        return false;
    }

    let sqrtd = discriminant.sqrt();
    let root1 = (-half_b - sqrtd) / a;
    let root2 = (-half_b + sqrtd) / a;

    root1 >= 0.0 || root2 >= 0.0
}

#[cfg(test)]
mod tests {
    use crate::vec3::Vec3;
    use super::*;

    #[test]
    fn test_hit_sphere() {
        // Ray pointing at sphere should hit
        let ray = Ray::new(
            Point3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, -1.0)
        );
        let center = Point3::new(0.0, 0.0, -1.0);
        let radius = 0.5;
        assert!(hit_sphere(center, radius, &ray));

        // Ray pointing away from sphere should miss
        let ray_miss = Ray::new(
            Point3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 1.0)
        );
        assert!(!hit_sphere(center, radius, &ray_miss));
    }
}
