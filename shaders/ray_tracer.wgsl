// Ray Tracing in WGSL for cross-platform compatibility

// Constants
const PI: f32 = 3.1415926535897932385;
const INFINITY: f32 = 1.0e30;
const EPSILON: f32 = 0.0001;

// Camera uniform buffer
struct Camera {
    origin: vec4<f32>,
    lower_left_corner: vec4<f32>,
    horizontal: vec4<f32>, 
    vertical: vec4<f32>,
    samples_per_pixel: u32,
    max_depth: u32,
    image_width: u32,
    image_height: u32,
}

// Bind group layout
@group(0) @binding(0) var<uniform> camera: Camera;
@group(0) @binding(1) var<storage, read_write> output: array<u32>;

// Random number generation on GPU
struct RandomState {
    seed: u32,
}

var<private> rand_state: RandomState;

// PCG random number generator
fn init_rand(pixel_idx: u32, frame: u32) {
    rand_state.seed = pixel_idx * 719393u + frame * 4731u;
}

fn pcg_hash(input: u32) -> u32 {
    var state = input * 747796405u + 2891336453u;
    var word = ((state >> ((state >> 28u) + 4u)) ^ state) * 277803737u;
    return (word >> 22u) ^ word;
}

fn rand_next_u32() -> u32 {
    rand_state.seed = pcg_hash(rand_state.seed);
    return rand_state.seed;
}

fn rand_float() -> f32 {
    return f32(rand_next_u32()) / 4294967295.0;
}

fn rand_float_range(min: f32, max: f32) -> f32 {
    return min + (max - min) * rand_float();
}

// Utility structs
struct Ray {
    origin: vec3<f32>,
    direction: vec3<f32>,
}

struct Material {
    type: u32,       // 0 = lambertian, 1 = metal, 2 = dielectric
    albedo: vec3<f32>,
    fuzz: f32,       // For metal
    ref_idx: f32,    // For dielectric
}

struct HitRecord {
    p: vec3<f32>,
    normal: vec3<f32>,
    t: f32,
    front_face: bool,
    material: Material,
}

struct Sphere {
    center: vec3<f32>,
    radius: f32,
    material: Material,
}

// Materials
const LAMBERTIAN: u32 = 0u;
const METAL: u32 = 1u;
const DIELECTRIC: u32 = 2u;

// Vector utility functions
fn random_in_unit_sphere() -> vec3<f32> {
    while true {
        let p = vec3<f32>(
            rand_float_range(-1.0, 1.0), 
            rand_float_range(-1.0, 1.0), 
            rand_float_range(-1.0, 1.0)
        );
        if dot(p, p) < 1.0 {
            return p;
        }
    }
    return vec3<f32>(0.0); // Unreachable, but needed for compiler
}

fn random_unit_vector() -> vec3<f32> {
    return normalize(random_in_unit_sphere());
}

fn random_in_hemisphere(normal: vec3<f32>) -> vec3<f32> {
    let in_unit_sphere = random_in_unit_sphere();
    if dot(in_unit_sphere, normal) > 0.0 {
        return in_unit_sphere;
    } else {
        return -in_unit_sphere;
    }
}

fn reflect(v: vec3<f32>, n: vec3<f32>) -> vec3<f32> {
    return v - 2.0 * dot(v, n) * n;
}

fn refract(uv: vec3<f32>, n: vec3<f32>, etai_over_etat: f32) -> vec3<f32> {
    let cos_theta = min(dot(-uv, n), 1.0);
    let r_out_perp = etai_over_etat * (uv + cos_theta * n);
    let r_out_parallel = -sqrt(abs(1.0 - dot(r_out_perp, r_out_perp))) * n;
    return r_out_perp + r_out_parallel;
}

fn schlick(cosine: f32, ref_idx: f32) -> f32 {
    var r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;
    return r0 + (1.0 - r0) * pow((1.0 - cosine), 5.0);
}

// Ray functions
fn ray_at(ray: Ray, t: f32) -> vec3<f32> {
    return ray.origin + t * ray.direction;
}

fn ray_color(ray: Ray, depth: i32) -> vec3<f32> {
    var cur_ray = ray;
    var cur_attenuation = vec3<f32>(1.0);
    
    // Example sphere - hardcoded for simplicity
    // In a real implementation, these would be passed in via a storage buffer
    let sphere1 = Sphere(
        vec3<f32>(0.0, 0.0, -1.0),  // center
        0.5,                        // radius
        Material(
            LAMBERTIAN,             // type
            vec3<f32>(0.7, 0.3, 0.3), // albedo
            0.0,                    // fuzz (not used)
            0.0                     // ref_idx (not used)
        )
    );
    
    let sphere2 = Sphere(
        vec3<f32>(0.0, -100.5, -1.0), // center
        100.0,                      // radius
        Material(
            LAMBERTIAN,             // type
            vec3<f32>(0.8, 0.8, 0.0), // albedo
            0.0,                    // fuzz (not used)
            0.0                     // ref_idx (not used)
        )
    );
    
    let sphere3 = Sphere(
        vec3<f32>(1.0, 0.0, -1.0),  // center
        0.5,                        // radius
        Material(
            METAL,                  // type
            vec3<f32>(0.8, 0.6, 0.2), // albedo
            0.2,                    // fuzz
            0.0                     // ref_idx (not used)
        )
    );
    
    let sphere4 = Sphere(
        vec3<f32>(-1.0, 0.0, -1.0), // center
        0.5,                        // radius
        Material(
            DIELECTRIC,             // type
            vec3<f32>(1.0),         // albedo
            0.0,                    // fuzz (not used)
            1.5                     // ref_idx
        )
    );
    
    let sphere5 = Sphere(
        vec3<f32>(-1.0, 0.0, -1.0), // center
        -0.45,                      // radius (negative for hollow sphere)
        Material(
            DIELECTRIC,             // type
            vec3<f32>(1.0),         // albedo
            0.0,                    // fuzz (not used)
            1.5                     // ref_idx
        )
    );
    
    for (var d: i32 = 0; d < depth; d++) {
        var rec: HitRecord;
        var hit_anything = false;
        var closest_so_far = INFINITY;
        
        // Check intersection with sphere1
        if hit_sphere(sphere1, cur_ray, 0.001, closest_so_far, &rec) {
            hit_anything = true;
            closest_so_far = rec.t;
        }
        
        // Check intersection with sphere2
        if hit_sphere(sphere2, cur_ray, 0.001, closest_so_far, &rec) {
            hit_anything = true;
            closest_so_far = rec.t;
        }
        
        // Check intersection with sphere3
        if hit_sphere(sphere3, cur_ray, 0.001, closest_so_far, &rec) {
            hit_anything = true;
            closest_so_far = rec.t;
        }
        
        // Check intersection with sphere4
        if hit_sphere(sphere4, cur_ray, 0.001, closest_so_far, &rec) {
            hit_anything = true;
            closest_so_far = rec.t;
        }
        
        // Check intersection with sphere5
        if hit_sphere(sphere5, cur_ray, 0.001, closest_so_far, &rec) {
            hit_anything = true;
            closest_so_far = rec.t;
        }
        
        if hit_anything {
            var scattered = Ray(vec3<f32>(0.0), vec3<f32>(0.0));
            var attenuation = vec3<f32>(0.0);
            
            if scatter(rec.material, cur_ray, rec, &attenuation, &scattered) {
                cur_attenuation *= attenuation;
                cur_ray = scattered;
            } else {
                return vec3<f32>(0.0);
            }
        } else {
            let unit_direction = normalize(cur_ray.direction);
            let t = 0.5 * (unit_direction.y + 1.0);
            let background = (1.0 - t) * vec3<f32>(1.0) + t * vec3<f32>(0.5, 0.7, 1.0);
            return cur_attenuation * background;
        }
    }
    
    // If we've exceeded the ray bounce limit, no more light is gathered
    return vec3<f32>(0.0);
}

fn hit_sphere(sphere: Sphere, ray: Ray, t_min: f32, t_max: f32, rec: ptr<function, HitRecord>) -> bool {
    let oc = ray.origin - sphere.center;
    let a = dot(ray.direction, ray.direction);
    let half_b = dot(oc, ray.direction);
    let c = dot(oc, oc) - sphere.radius * sphere.radius;
    
    let discriminant = half_b * half_b - a * c;
    if discriminant < 0.0 {
        return false;
    }
    
    let sqrtd = sqrt(discriminant);
    
    // Find the nearest root that lies in the acceptable range
    var root = (-half_b - sqrtd) / a;
    if root < t_min || t_max < root {
        root = (-half_b + sqrtd) / a;
        if root < t_min || t_max < root {
            return false;
        }
    }
    
    (*rec).t = root;
    (*rec).p = ray_at(ray, root);
    let outward_normal = ((*rec).p - sphere.center) / sphere.radius;
    (*rec).front_face = dot(ray.direction, outward_normal) < 0.0;
    (*rec).normal = select(-outward_normal, outward_normal, (*rec).front_face);
    (*rec).material = sphere.material;
    
    return true;
}

fn scatter(material: Material, ray_in: Ray, rec: HitRecord, attenuation: ptr<function, vec3<f32>>, scattered: ptr<function, Ray>) -> bool {
    switch material.type {
        case LAMBERTIAN: {
            let scatter_direction = rec.normal + random_unit_vector();
            
            // Catch degenerate scatter direction
            let scatter_dir = select(
                scatter_direction,
                rec.normal,
                dot(scatter_direction, scatter_direction) < EPSILON
            );
            
            (*scattered) = Ray(rec.p, scatter_dir);
            (*attenuation) = material.albedo;
            return true;
        }
        case METAL: {
            let reflected = reflect(normalize(ray_in.direction), rec.normal);
            (*scattered) = Ray(
                rec.p,
                reflected + material.fuzz * random_in_unit_sphere()
            );
            (*attenuation) = material.albedo;
            return dot((*scattered).direction, rec.normal) > 0.0;
        }
        case DIELECTRIC: {
            (*attenuation) = vec3<f32>(1.0);
            let refraction_ratio = select(
                material.ref_idx,
                1.0 / material.ref_idx,
                rec.front_face
            );
            
            let unit_direction = normalize(ray_in.direction);
            let cos_theta = min(dot(-unit_direction, rec.normal), 1.0);
            let sin_theta = sqrt(1.0 - cos_theta * cos_theta);
            
            let cannot_refract = refraction_ratio * sin_theta > 1.0;
            let direction = select(
                refract(unit_direction, rec.normal, refraction_ratio),
                reflect(unit_direction, rec.normal),
                cannot_refract || schlick(cos_theta, refraction_ratio) > rand_float()
            );
            
            (*scattered) = Ray(rec.p, direction);
            return true;
        }
        default: {
            return false;
        }
    }
}

// Main compute shader entry point
@compute @workgroup_size(16, 16, 1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let x = global_id.x;
    let y = global_id.y;
    
    // Check if within bounds
    if x >= camera.image_width || y >= camera.image_height {
        return;
    }
    
    // Initialize random state
    let pixel_index = y * camera.image_width + x;
    init_rand(pixel_index, 0u);
    
    var pixel_color = vec3<f32>(0.0, 0.0, 0.0);
    let samples = camera.samples_per_pixel;
    let max_depth = i32(camera.max_depth);
    
    for (var s = 0u; s < samples; s++) {
        let u = (f32(x) + rand_float()) / f32(camera.image_width - 1u);
        let v = (f32(y) + rand_float()) / f32(camera.image_height - 1u);
        
        let direction = vec3<f32>(camera.lower_left_corner.xyz) +
                        u * vec3<f32>(camera.horizontal.xyz) +
                        v * vec3<f32>(camera.vertical.xyz) -
                        vec3<f32>(camera.origin.xyz);
        
        let ray = Ray(
            vec3<f32>(camera.origin.xyz),
            direction
        );
        
        pixel_color += ray_color(ray, max_depth);
    }
    
    // Calculate the final color with gamma correction
    var r = pixel_color.x / f32(samples);
    var g = pixel_color.y / f32(samples);
    var b = pixel_color.z / f32(samples);
    
    // Gamma-correct for gamma=2.0
    r = sqrt(r);
    g = sqrt(g);
    b = sqrt(b);
    
    // Convert to 8-bit color and pack into RGBA
    let ir = u32(clamp(r * 255.0, 0.0, 255.0));
    let ig = u32(clamp(g * 255.0, 0.0, 255.0));
    let ib = u32(clamp(b * 255.0, 0.0, 255.0));
    
    // Store the result (using RGBA format)
    output[pixel_index] = (255u << 24) | (ib << 16) | (ig << 8) | ir;
}