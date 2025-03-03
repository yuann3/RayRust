use raytracer::bababoi::{random_double, random_double_range};
use raytracer::camera::Camera;
use raytracer::hittable_list::HittableList;
use raytracer::material::{Dielectric, Lambertian, Metal};
use raytracer::sphere::Sphere;
use raytracer::vec3::{Color, Point3, Vec3};
use std::env;
use std::io;

fn main() -> io::Result<()> {
    // Parse command line arguments
    let args: Vec<String> = env::args().collect();
    let mut use_gpu = false;
    let mut output_file = None;
    
    // Simple argument parsing
    for arg in &args[1..] {
        match arg.as_str() {
            "--gpu" | "-g" => use_gpu = true,
            "-o" | "--output" => {
                // Next argument is the output file
                let index = args.iter().position(|a| a == arg).unwrap();
                if index + 1 < args.len() {
                    output_file = Some(args[index + 1].clone());
                }
            }
            _ => {
                // Check if it's an output file after -o flag
                if args.iter().position(|a| a == "-o" || a == "--output").unwrap_or(args.len()) 
                   == args.iter().position(|a| a == arg).unwrap_or(args.len()) - 1 {
                    continue;
                }
                
                if arg.starts_with("-") {
                    eprintln!("Unknown option: {}", arg);
                    return Ok(());
                }
            }
        }
    }
    
    // Print rendering mode
    if use_gpu {
        eprintln!("Using GPU acceleration");
    } else {
        eprintln!("Using CPU rendering");
    }
    
    // Create the world
    let mut world = HittableList::new();

    let ground_material = Lambertian::new(Color::new(0.5, 0.5, 0.5));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_double();
            let center = Point3::new(
                a as f64 + 0.9 * random_double(),
                0.2,
                b as f64 + 0.9 * random_double(),
            );

            // Ensure spheres don't overlap with the three big spheres
            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                // Choose random material
                if choose_mat < 0.8 {
                    // Diffuse (80% chance)
                    let albedo = Color::random() * Color::random();
                    let sphere_material = Lambertian::new(albedo);
                    world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                } else if choose_mat < 0.95 {
                    // Metal (15% chance)
                    let albedo = Color::random_range(0.5, 1.0);
                    let fuzz = random_double_range(0.0, 0.5);
                    let sphere_material = Metal::new(albedo, fuzz);
                    world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    // Glass (5% chance)
                    let sphere_material = Dielectric::new(1.5);
                    world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }
    }

    // Big glass sphere in the center
    let material1 = Dielectric::new(1.5);
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    // Big brown Lambertian sphere on the left
    let material2 = Lambertian::new(Color::new(0.4, 0.2, 0.1));
    world.add(Box::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    // Big metallic sphere on the right
    let material3 = Metal::new(Color::new(0.7, 0.6, 0.5), 0.0);
    world.add(Box::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    // Camera setup
    let mut cam = Camera::new();
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 1200;
    cam.samples_per_pixel = 500;
    cam.max_depth = 50;
    cam.vfov = 20.0;
    cam.lookfrom = Point3::new(13.0, 2.0, 3.0);
    cam.lookat = Point3::new(0.0, 0.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);
    cam.defocus_angle = 0.6;
    cam.focus_dist = 10.0;
    cam.use_gpu = use_gpu;

    // Render the scene
    match output_file {
        Some(filename) => cam.render_to_file(&world, &filename)?,
        None => cam.render(&world)?,
    }

    Ok(())
}
