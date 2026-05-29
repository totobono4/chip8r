use pixels::{Pixels, SurfaceTexture};
use winit::application::ApplicationHandler;
use winit::dpi::{LogicalSize, Size};
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};

const PIXELS_WIDTH: u32 = 64;
const PIXELS_HEIGHT: u32 = 32;
const WINDOW_SCALE: u32 = 10;
const WINDOW_WIDTH: u32 = PIXELS_WIDTH * WINDOW_SCALE;
const WINDOW_HEIGHT: u32 = PIXELS_HEIGHT * WINDOW_SCALE;
const DEFAULT_COLOR: [u8; 4] = [0, 0, 0, 255];

#[derive(Default)]

struct App {
    window: Option<&'static Window>,
    pixels: Option<Pixels<'static>>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = event_loop.create_window(Window::default_attributes().with_title("chip8r").with_inner_size(Size::Logical(LogicalSize::new(WINDOW_WIDTH as f64, WINDOW_HEIGHT as f64)))).unwrap();
        let window_ref: &'static Window = Box::leak(Box::new(window));
        let size = window_ref.inner_size();
        let surface = SurfaceTexture::new(size.width, size.height, window_ref);
        let pixels = Pixels::new(PIXELS_WIDTH, PIXELS_HEIGHT, surface).unwrap();
        self.window = Some(window_ref);
        self.pixels = Some(pixels);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                println!("Exiting chip8r...");
                event_loop.exit();
            },
            WindowEvent::RedrawRequested => {
                if let Some(pixels) = &mut self.pixels {
                    let frame = pixels.frame_mut();
                    for pixel in frame.chunks_exact_mut(4) {
                        pixel[0] = DEFAULT_COLOR[0]; // R
                        pixel[1] = DEFAULT_COLOR[1]; // G
                        pixel[2] = DEFAULT_COLOR[2]; // B
                        pixel[3] = DEFAULT_COLOR[3]; // A
                    }
                    pixels.render().unwrap();
                }
                if let Some(window) = self.window {
                    window.request_redraw();
                }
            }
            _ => (),
        }
    }
}

pub fn init() {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app: App = App::default();
    let _ = event_loop.run_app(&mut app);
}
