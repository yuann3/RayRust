use crate::ray::Ray;
use crate::vec3::Point3;

pub fn hit_sphere(center: Point3, radius: f64, ray: &Ray) -> f64 {
    let oc = ray.origin() - center;
    let a = ray.direction().length_squared();
    let h = oc.dot(&ray.direction());
    let c = oc.dot(&oc) - radius * radius;
    let discriminant = h * h - a * c;

    if discriminant < 0.0 {
        -1.0
    } else {
        (-h - discriminant.sqrt()) / a
    }
}
