use pixels::{Pixels, SurfaceTexture};
use winit::application::ApplicationHandler;
use winit::dpi::{LogicalSize, Size};
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};
use std::sync::Arc;
use std::time;
use crate::consts;
use crate::chip8;

const WINDOW_TITLE: &str = "chip8r";
const WINDOW_SCALE: u32 = 10;
const WINDOW_WIDTH: u32 = consts::DISPLAY_WIDTH as u32 * WINDOW_SCALE;
const WINDOW_HEIGHT: u32 = consts::DISPLAY_HEIGHT as u32 * WINDOW_SCALE;
 
struct App {
    window: Option<Arc<Window>>,
    pixels: Option<Pixels<'static>>,
    chip8: chip8::Chip8,
    last_tick: time::Instant,
}

impl App {
    pub fn new(chip8: chip8::Chip8) -> Self {
        Self {
            window: None,
            pixels: None,
            chip8: chip8,
            last_tick: time::Instant::now(),
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = Arc::new(event_loop.create_window(
            Window::default_attributes()
                .with_title(WINDOW_TITLE)
                .with_inner_size(Size::Logical(LogicalSize::new(
                    WINDOW_WIDTH as f64,
                    WINDOW_HEIGHT as f64
                )))
            )
        .unwrap());
        let size = window.inner_size();
        let surface = SurfaceTexture::new(size.width, size.height, Arc::clone(&window));
        let pixels = Pixels::new(
            consts::DISPLAY_WIDTH as u32,
            consts::DISPLAY_HEIGHT as u32,
            surface
        ).unwrap();

        self.window = Some(window);
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
                    let chip8_display_buffer = self.chip8.get_display_buffer();
                    for i in 0..chip8_display_buffer.len() {
                        frame[i*4+0] = chip8_display_buffer[i][0];
                        frame[i*4+1] = chip8_display_buffer[i][1];
                        frame[i*4+2] = chip8_display_buffer[i][2];
                        frame[i*4+3] = chip8_display_buffer[i][3];
                    }
                    pixels.render().unwrap();
                }
                if let Some(window) = &self.window {
                    window.request_redraw();
                }
            },
            _ => (),
        }
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        let now = time::Instant::now();
        if now < self.last_tick + time::Duration::from_secs_f32(consts::FREQUENCY) { return; }
        self.last_tick = now;
        self.chip8.tick();
    }
}

pub fn run(chip8: chip8::Chip8) {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app: App = App::new(chip8);
    let _ = event_loop.run_app(&mut app);
}
