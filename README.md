![image](https://github.com/user-attachments/assets/67c509ac-c1f3-4d3b-9edc-e1b77f277197)

just a simple ray tracing in Rust; I wrote it for fun and learning. no GPU acceleration (yet), so do not take it seriously

you might want to lower the `samples_per_pixel` in `main.rs`, i set it to 500 just to render the thumbnail

```rust
...

 // Camera setup
    let mut cam = Camera::new();
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 1200;
    cam.samples_per_pixel = 500; // You may lower down this, 100 - 200 its a good option
    cam.max_depth = 50;
    cam.vfov = 20.0;
    cam.lookfrom = Point3::new(13.0, 2.0, 3.0);
    cam.lookat = Point3::new(0.0, 0.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);
    cam.defocus_angle = 0.6;
    cam.focus_dist = 10.0;

...
```

to build and run this, clone it and use:

```bash
./run
```

or you can run:

```bash
cargo run > image.ppm
```

which will generate a ppm format file

you can open the generated image using any compatible image viewer:

```bash
open image.ppm
```

