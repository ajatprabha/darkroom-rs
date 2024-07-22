use std::sync::Arc;
use async_trait::async_trait;
use image::{DynamicImage, GenericImageView, ImageBuffer, ImageFormat, Rgba, RgbaImage, RgbImage};
use wgpu::util::DeviceExt;
use crate::processor::{Image, Processor, Error};
use crate::processor::gpuprocs::backend::GPUBackend;
use crate::processor::gpuprocs::utils::capitalize;
use crate::processor::error::Error::{BufferError, GPUError, ImageBufferCreateError};

const GAUSSIAN_BLUR_SHADER: &str = include_str!("shaders/gaussian_blur.wgsl");
pub const MAX_BLUR_RADIUS: u16 = 2000;

pub struct Blur {
    pub radius: u16,
    pub gpu: Arc<GPUBackend>,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct KernelHeader {
    sum: f32,
    size: u32,
}

struct Kernel {
    sum: f32,
    values: Vec<f32>,
}

impl Blur {
    fn kernel(&self) -> Kernel {
        let radius = self.radius as i32;
        let sigma = radius as f32 / 3.0;
        let mut values = Vec::new();
        let mut sum = 0.0;

        for y in -radius..=radius {
            for x in -radius..=radius {
                let distance = (x * x + y * y) as f32;
                let value = (-distance / (2.0 * sigma * sigma)).exp() / (2.0 * std::f32::consts::PI * sigma * sigma);
                values.push(value);
                sum += value;
            }
        }

        Kernel {
            sum,
            values,
        }
    }

    async fn blur(&self, image: &DynamicImage) -> Result<DynamicImage, Error> {
        let rgba_image = image.to_rgba8();
        let (width, height) = rgba_image.dimensions();
        let input_data = rgba_image.into_raw();

        let (device, queue) = (&self.gpu.device, &self.gpu.queue);

        let cs_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Gaussian Blur Shader"),
            source: wgpu::ShaderSource::Wgsl(GAUSSIAN_BLUR_SHADER.into()),
        });

        let kernel = self.kernel();
        let kernel_size = (kernel.values.len() as f32).sqrt() as u32;
        let kernel_header = KernelHeader {
            sum: kernel.sum,
            size: kernel_size,
        };

        let kernel_data: Vec<f32> = kernel.values.clone();
        let kernel_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Kernel Buffer"),
            contents: bytemuck::cast_slice(&[kernel_header])
                .iter()
                .chain(bytemuck::cast_slice(&kernel_data))
                .map(|x| *x)
                .collect::<Vec<u8>>()
                .as_slice(),
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
        });

        let texture_size = wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        };
        let input_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Input Texture"),
            size: texture_size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        let input_texture_view = input_texture.create_view(&wgpu::TextureViewDescriptor::default());

        let output_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Output Texture"),
            size: texture_size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::STORAGE_BINDING | wgpu::TextureUsages::COPY_SRC,
            view_formats: &[],
        });

        let output_texture_view = output_texture.create_view(&wgpu::TextureViewDescriptor::default());

        let bind_group_layout_0 = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Bind Group Layout 0"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
        });

        let bind_group_layout_1 = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Bind Group Layout 1"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: false },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::StorageTexture {
                        access: wgpu::StorageTextureAccess::WriteOnly,
                        format: wgpu::TextureFormat::Rgba8Unorm,
                        view_dimension: wgpu::TextureViewDimension::D2,
                    },
                    count: None,
                },
            ],
        });

        let bind_group_0 = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Bind Group 0"),
            layout: &bind_group_layout_0,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: kernel_buffer.as_entire_binding(),
                },
            ],
        });

        let bind_group_1 = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Bind Group 1"),
            layout: &bind_group_layout_1,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&input_texture_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(&output_texture_view),
                },
            ],
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Pipeline Layout"),
            bind_group_layouts: &[&bind_group_layout_0, &bind_group_layout_1],
            push_constant_ranges: &[],
        });

        let compute_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("Compute Pipeline"),
            layout: Some(&pipeline_layout),
            module: &cs_module,
            entry_point: "main",
            compilation_options: wgpu::PipelineCompilationOptions::default(),
            cache: None,
        });

        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Command Encoder"),
        });

        let input_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Input Buffer"),
            contents: input_data.as_slice(),
            usage: wgpu::BufferUsages::COPY_SRC,
        });

        let bytes_per_row = ((4 * width + 255) / 256) * 256;
        let buffer_size = input_data.len() as u64;

        encoder.copy_buffer_to_texture(
            wgpu::ImageCopyBuffer {
                buffer: &input_buffer,
                layout: wgpu::ImageDataLayout {
                    offset: 0,
                    bytes_per_row: Some(bytes_per_row),
                    rows_per_image: Some(height),
                },
            },
            wgpu::ImageCopyTexture {
                texture: &input_texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            texture_size,
        );

        {
            let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("Compute Pass"),
                timestamp_writes: None,
            });
            compute_pass.set_pipeline(&compute_pipeline);
            compute_pass.set_bind_group(0, &bind_group_0, &[]);
            compute_pass.set_bind_group(1, &bind_group_1, &[]);
            compute_pass.dispatch_workgroups((width + 7) / 8, (height + 7) / 8, 1);
        }

        // Copy result to a buffer
        let temp_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Temp Buffer"),
            size: (bytes_per_row * height) as u64,
            usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        encoder.copy_texture_to_buffer(
            wgpu::ImageCopyTexture {
                texture: &output_texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            wgpu::ImageCopyBuffer {
                buffer: &temp_buffer,
                layout: wgpu::ImageDataLayout {
                    offset: 0,
                    bytes_per_row: Some(bytes_per_row),
                    rows_per_image: Some(height),
                },
            },
            texture_size,
        );

        queue.submit(Some(encoder.finish()));

        let buffer_slice = temp_buffer.slice(..);
        let (sender, receiver) = tokio::sync::oneshot::channel();
        buffer_slice.map_async(wgpu::MapMode::Read, move |result| {
            sender.send(result).unwrap();
        });

        device.poll(wgpu::Maintain::Wait);
        receiver.await.unwrap().map_err(BufferError)?;

        let data = buffer_slice.get_mapped_range();
        let result: Vec<u8> = data.to_vec();

        let blurred_image = ImageBuffer::<Rgba<u8>, _>::from_raw(width, height, result)
            .ok_or(ImageBufferCreateError)?;
        let dynamic_image = DynamicImage::ImageRgba8(blurred_image);

        drop(data);
        temp_buffer.unmap();

        Ok(dynamic_image)
    }
}

#[async_trait]
impl Processor for Blur {
    async fn process(&self, image: &mut Image) -> Result<(), Error> {
        if self.radius == 0 { return Ok(()); }

        let blurred = self
            .blur(image)
            .await?;

        image.extend(blurred);

        Ok(())
    }
}
