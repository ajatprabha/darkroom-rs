use wgpu::{DeviceDescriptor, RequestAdapterOptions};
use crate::error::Error::GPU;
use crate::prelude::Result;
use crate::processor::gpuprocs::error::Error::{DeviceError, AdapterNotFound};

pub struct GPUBackend {
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
}

impl GPUBackend {
    pub async fn new() -> Result<Self> {
        let instance = wgpu::Instance::default();
        {
            println!("Available adapters:");
            for a in instance.enumerate_adapters(wgpu::Backends::all()) {
                println!("    {:?}", a.get_info())
            }
        }
        let adapter = instance
            .request_adapter(&RequestAdapterOptions::default())
            .await
            .ok_or_else(|| GPU(AdapterNotFound))?;

        println!("Using adapter: {:?}", adapter.get_info());

        let (device, queue) = adapter
            .request_device(
                &DeviceDescriptor {
                    label: None,
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::default(),
                    memory_hints: wgpu::MemoryHints::MemoryUsage,
                },
                None,
            )
            .await
            .map_err(|e| GPU(DeviceError(e)))?;

        Ok(Self { device, queue })
    }
}
