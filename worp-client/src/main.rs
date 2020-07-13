use anyhow::Result;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

fn main() -> Result<()> {
    FmtSubscriber::builder().with_max_level(Level::INFO).init();

    let event_loop = EventLoop::new();
    let _window = WindowBuilder::new().with_title("WORP").build(&event_loop)?;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                *control_flow = ControlFlow::Exit;
                info!("Closing application.");
            }
            _ => (),
        }
    });
}
