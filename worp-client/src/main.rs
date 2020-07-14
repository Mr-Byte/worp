use anyhow::Result;
use tracing::{error, info, Level};
use tracing_subscriber::FmtSubscriber;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

fn main() -> Result<()> {
    FmtSubscriber::builder().with_max_level(Level::INFO).init();

    let event_loop = EventLoop::new();
    // TODO: Initialize window with config settings.
    let window = WindowBuilder::new().with_title("WORP").with_visible(false).build(&event_loop)?;

    let mut renderer = futures_executor::block_on(renderer::Renderer::new(&window))?;

    window.set_visible(true);

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        match event {
            Event::WindowEvent {
                event: WindowEvent::Resized(size),
                ..
            } => {
                renderer.resize(size.width, size.height);
            }
            Event::RedrawRequested(_) => {
                if let Err(error) = renderer.draw_frame() {
                    error!("Error drawing frame: {}", error);
                    *control_flow = ControlFlow::Exit;
                }
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                *control_flow = ControlFlow::Exit;
                info!("Closing application.")
            }
            _ => {}
        }
    });
}
