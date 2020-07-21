use anyhow::{Context as _, Result};
use wgpu::{
    Adapter, BackendBit, Color, Device, DeviceDescriptor, Features, Instance, LoadOp, Operations, Queue, RenderPassColorAttachmentDescriptor,
    Surface, SwapChain, SwapChainDescriptor,
};
use winit::window::Window;

pub struct Renderer {
    window_surface: Surface,
    _adapter: Adapter,
    device: Device,
    queue: Queue,
    swap_chain_descriptor: SwapChainDescriptor,
    swap_chain: SwapChain,
}

impl Renderer {
    pub async fn new(window: &Window) -> Result<Self> {
        let size = window.inner_size();
        let instance = Instance::new(BackendBit::PRIMARY);
        let window_surface = unsafe { instance.create_surface(window) };
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::Default,
                compatible_surface: Some(&window_surface),
            })
            .await
            .context("Failed to find appropriate adapter.")?;

        let (device, queue) = adapter
            .request_device(
                &DeviceDescriptor {
                    features: Features::empty(),
                    limits: wgpu::Limits::default(),
                    shader_validation: true,
                },
                None,
            )
            .await
            .map_err(|_| RequestDeviceError)?;

        let swap_chain_descriptor = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Immediate,
        };

        let swap_chain = device.create_swap_chain(&window_surface, &swap_chain_descriptor);

        Ok(Self {
            window_surface,
            _adapter: adapter,
            device,
            queue,
            swap_chain_descriptor,
            swap_chain,
        })
    }

    pub fn device(&mut self) -> &mut Device {
        &mut self.device
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.swap_chain_descriptor.width = width;
        self.swap_chain_descriptor.height = height;
        self.swap_chain = self.device.create_swap_chain(&self.window_surface, &self.swap_chain_descriptor);
    }

    pub fn draw_frame(&mut self) -> Result<()> {
        let frame = self.swap_chain.get_current_frame().map_err(|_| SwapChainError)?;
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        let _ = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            color_attachments: &[RenderPassColorAttachmentDescriptor {
                attachment: &frame.output.view,
                resolve_target: None,
                ops: Operations {
                    load: LoadOp::Clear(Color::BLACK),
                    store: true,
                },
            }],
            depth_stencil_attachment: None,
        });

        self.queue.submit(Some(encoder.finish()));

        Ok(())
    }
}

#[derive(thiserror::Error, Debug)]
#[error("Failed to acquire next swap chain framebuffer.")]
pub struct SwapChainError;

#[derive(thiserror::Error, Debug)]
#[error("Failed to request device.")]
pub struct RequestDeviceError;
