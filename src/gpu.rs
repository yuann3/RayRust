use crate::hittable::Hittable;
use crate::vec3::{Point3, Vec3};
use bytemuck::{Pod, Zeroable};
use wgpu::util::DeviceExt;
use winit::dpi::PhysicalSize;

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct CameraUniform {
    origin: [f32; 4],
    lower_left_corner: [f32; 4],
    horizontal: [f32; 4],
    vertical: [f32; 4],
    samples_per_pixel: u32,
    max_depth: u32,
    image_width: u32,
    image_height: u32,
}

// This struct holds all the GPU state
pub struct GpuRenderer {
    device: wgpu::Device,
    queue: wgpu::Queue,
    size: winit::dpi::PhysicalSize<u32>,
    pipeline: wgpu::ComputePipeline,
    bind_group: wgpu::BindGroup,
    output_buffer: wgpu::Buffer,
    camera_buffer: wgpu::Buffer,
    camera_uniform: CameraUniform,
}

impl GpuRenderer {
    pub async fn new(size: winit::dpi::PhysicalSize<u32>) -> Self {
        // Create wgpu instance
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        // Get adapter for GPU - headless
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: None,
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        // Get device and queue
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: Some("Ray Tracing Device"),
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::default(),
                },
                None,
            )
            .await
            .unwrap();

        // Create default camera uniform
        let camera_uniform = CameraUniform {
            origin: [0.0, 0.0, 0.0, 1.0],
            lower_left_corner: [-2.0, -1.0, -1.0, 1.0],
            horizontal: [4.0, 0.0, 0.0, 0.0],
            vertical: [0.0, 2.0, 0.0, 0.0],
            samples_per_pixel: 10,
            max_depth: 10,
            image_width: size.width,
            image_height: size.height,
        };

        // Create camera buffer
        let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera Buffer"),
            contents: bytemuck::cast_slice(&[camera_uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        // Create output buffer (for storing render results)
        let output_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Output Buffer"),
            size: (size.width * size.height * 4) as u64, // RGBA format
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
            mapped_at_creation: false,
        });

        // Create shader module from inline WGSL shader
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Ray Tracing Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../shaders/ray_tracer.wgsl").into()),
        });

        // Create binding layout
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Bind Group Layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: false },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
        });

        // Create bind group
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Bind Group"),
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: camera_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: output_buffer.as_entire_binding(),
                },
            ],
        });

        // Create compute pipeline
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Pipeline Layout"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("Compute Pipeline"),
            layout: Some(&pipeline_layout),
            module: &shader,
            entry_point: "main",
        });

        Self {
            device,
            queue,
            size,
            pipeline,
            bind_group,
            output_buffer,
            camera_buffer,
            camera_uniform,
        }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            
            // Also update camera settings
            self.camera_uniform.image_width = new_size.width;
            self.camera_uniform.image_height = new_size.height;
            self.queue.write_buffer(
                &self.camera_buffer,
                0,
                bytemuck::cast_slice(&[self.camera_uniform]),
            );

            // Recreate output buffer
            self.output_buffer = self.device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("Output Buffer"),
                size: (new_size.width * new_size.height * 4) as u64,
                usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
                mapped_at_creation: false,
            });
        }
    }

    pub fn update_camera(
        &mut self,
        origin: Point3, 
        lower_left_corner: Point3,
        horizontal: Vec3, 
        vertical: Vec3,
        samples_per_pixel: i32, 
        max_depth: i32
    ) {
        // Convert from f64 to f32 for GPU compatibility
        self.camera_uniform = CameraUniform {
            origin: [origin.x() as f32, origin.y() as f32, origin.z() as f32, 1.0],
            lower_left_corner: [
                lower_left_corner.x() as f32,
                lower_left_corner.y() as f32,
                lower_left_corner.z() as f32,
                1.0,
            ],
            horizontal: [
                horizontal.x() as f32,
                horizontal.y() as f32,
                horizontal.z() as f32,
                0.0,
            ],
            vertical: [
                vertical.x() as f32,
                vertical.y() as f32,
                vertical.z() as f32,
                0.0,
            ],
            samples_per_pixel: samples_per_pixel as u32,
            max_depth: max_depth as u32,
            image_width: self.size.width,
            image_height: self.size.height,
        };

        self.queue.write_buffer(
            &self.camera_buffer,
            0,
            bytemuck::cast_slice(&[self.camera_uniform]),
        );
    }

    // We don't need to render to a window anymore
    #[allow(dead_code)]
    pub fn render(&mut self) {
        // Create command encoder
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        // Dispatch compute pass
        {
            let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("Ray Tracing Pass"),
                timestamp_writes: None,
            });
            compute_pass.set_pipeline(&self.pipeline);
            compute_pass.set_bind_group(0, &self.bind_group, &[]);
            
            // Dispatch workgroups
            let workgroup_count_x = (self.size.width + 15) / 16;
            let workgroup_count_y = (self.size.height + 15) / 16;
            compute_pass.dispatch_workgroups(workgroup_count_x, workgroup_count_y, 1);
        }

        // Create staging buffer to read results
        let staging_buffer = self.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Staging Buffer"),
            size: (self.size.width * self.size.height * 4) as u64,
            usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // Copy output to staging buffer
        encoder.copy_buffer_to_buffer(
            &self.output_buffer,
            0,
            &staging_buffer,
            0,
            (self.size.width * self.size.height * 4) as u64,
        );

        // Submit commands
        self.queue.submit(std::iter::once(encoder.finish()));
    }

    pub fn render_to_image(&mut self) -> image::RgbaImage {
        // Dispatch compute shader
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Compute Encoder"),
            });

        {
            let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("Ray Tracing Pass"),
                timestamp_writes: None,
            });
            compute_pass.set_pipeline(&self.pipeline);
            compute_pass.set_bind_group(0, &self.bind_group, &[]);
            
            // Dispatch workgroups
            let workgroup_count_x = (self.size.width + 15) / 16;
            let workgroup_count_y = (self.size.height + 15) / 16;
            compute_pass.dispatch_workgroups(workgroup_count_x, workgroup_count_y, 1);
        }

        // Create staging buffer to read results
        let staging_buffer = self.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Staging Buffer"),
            size: (self.size.width * self.size.height * 4) as u64,
            usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // Copy output to staging buffer
        encoder.copy_buffer_to_buffer(
            &self.output_buffer,
            0,
            &staging_buffer,
            0,
            (self.size.width * self.size.height * 4) as u64,
        );

        // Submit commands
        self.queue.submit(std::iter::once(encoder.finish()));

        // Read data from staging buffer
        let buffer_slice = staging_buffer.slice(..);
        let (sender, _receiver) = futures::channel::oneshot::channel();
        buffer_slice.map_async(wgpu::MapMode::Read, move |v| sender.send(v).unwrap());

        // Wait for the GPU to finish
        self.device.poll(wgpu::Maintain::Wait);

        // Create image from data
        let data = buffer_slice.get_mapped_range();
        let image = image::RgbaImage::from_raw(
            self.size.width, 
            self.size.height, 
            data.to_vec()
        ).unwrap();

        drop(data);
        staging_buffer.unmap();

        image
    }
}

pub async fn run_gpu_renderer(
    _world: &dyn Hittable,
    image_width: i32,
    image_height: i32,
    samples_per_pixel: i32,
    max_depth: i32,
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
) -> image::RgbaImage {
    let size = winit::dpi::PhysicalSize::new(image_width as u32, image_height as u32);

    // Create GPU renderer without a window
    let mut renderer = GpuRenderer::new(size).await;
    
    // Update camera parameters
    renderer.update_camera(
        origin,
        lower_left_corner, 
        horizontal,
        vertical,
        samples_per_pixel,
        max_depth,
    );
    
    // Render to image (without running event loop)
    renderer.render_to_image()
}