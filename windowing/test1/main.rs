#[cfg(feature = "gl")]
use gfx_backend_gl as gl_back;

use hal::format::{AsFormat, Rgba8Srgb as ColorFormat};
use hal::{Instance, window::Surface};
use hal::adapter::PhysicalDevice;

use winit;

fn main() {
    if let Err(e) = run() {
        eprintln!("{:?}", e);
    }
}

fn run() -> anyhow::Result<()> {
    // The event loop
    let mut event_loop = winit::event_loop::EventLoop::new();

    // The window builder
    let window_builder = winit::window::WindowBuilder::new()
        .with_inner_size(winit::dpi::LogicalSize::<u32>::from_physical(
            winit::dpi::PhysicalSize {
                width: 400u32,
                height: 400u32,
            },
            1.0,
        ))
        .with_title("My Renderer!");

    // The graphics context builder
    let context_builder = gl_back::config_context(
        gl_back::glutin::ContextBuilder::new(),
        ColorFormat::SELF,
        None,
    )
    .with_vsync(true);

    // The context in a window?
    let windowed_context = context_builder.build_windowed(window_builder, &event_loop)?;

    // Unsafe because the context must be dropped before the window
    let (context, window) = unsafe {
        windowed_context
            .make_current()
            .expect("Unable to make context current")
            .split()
    };

    // The drawing surface extracted from the context
    let surface = gl_back::Surface::from_context(context);

    // The devices or software constructs that profide graphics implementation
    let mut adapters = surface.enumerate_adapters();

    for adapter in &adapters {
        println!("{:#?}", adapter.info);
    }

    // Get the first adapter
    let mut adapter = adapters.remove(0);

    // let (mut device, mut queue_group) = adapter
    //     .open_with::<_, hal::queue::QueueType>(1, |family| surface.supports_queue_family(family))?;

    // Start the event loop so we can do stuff
    event_loop.run(move |event, _, control_flow| {
        *control_flow = winit::event_loop::ControlFlow::Wait;

        println!("{:?}", event);
    });

    Ok(())
}
