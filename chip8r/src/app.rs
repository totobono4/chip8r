use pixels::{Pixels, SurfaceTexture};
use winit::application::ApplicationHandler;
use winit::dpi::{LogicalSize, Size};
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::platform::scancode::PhysicalKeyExtScancode;
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
    last_draw: time::Instant,
    draw_next_frame: bool,
}

impl App {
    pub fn new(chip8: chip8::Chip8) -> Self {
        Self {
            window: None,
            pixels: None,
            chip8: chip8,
            last_tick: time::Instant::now(),
            last_draw: time::Instant::now(),
            draw_next_frame: false,
        }
    }

    pub fn handle_cpu(&mut self) {
        let now = time::Instant::now();
        if now < self.last_tick + time::Duration::from_secs_f32(consts::CPU_FREQUENCY) { return; }
        self.last_tick = now;

        self.chip8.tick();
    }

    pub fn handle_display(&mut self) {
        let now = time::Instant::now();
        if now < self.last_draw + time::Duration::from_secs_f32(consts::DISPLAY_FREQUENCY) { return; }
        self.last_draw = now;
        self.draw_next_frame = false;

        match &self.window {
            Some(window) => {
                window.request_redraw();
            }
            None => {}
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
            },
            WindowEvent::KeyboardInput { device_id: _, event, is_synthetic: _ } => {
                match event.physical_key.to_scancode() {
                    Some(scancode) => {
                        match scancode {
                            45 => { self.chip8.set_key(0x0, event.state.is_pressed()); }
                            02 => { self.chip8.set_key(0x1, event.state.is_pressed()); }
                            03 => { self.chip8.set_key(0x2, event.state.is_pressed()); }
                            04 => { self.chip8.set_key(0x3, event.state.is_pressed()); }
                            16 => { self.chip8.set_key(0x4, event.state.is_pressed()); }
                            17 => { self.chip8.set_key(0x5, event.state.is_pressed()); }
                            18 => { self.chip8.set_key(0x6, event.state.is_pressed()); }
                            30 => { self.chip8.set_key(0x7, event.state.is_pressed()); }
                            31 => { self.chip8.set_key(0x8, event.state.is_pressed()); }
                            32 => { self.chip8.set_key(0x9, event.state.is_pressed()); }
                            44 => { self.chip8.set_key(0xA, event.state.is_pressed()); }
                            46 => { self.chip8.set_key(0xB, event.state.is_pressed()); }
                            05 => { self.chip8.set_key(0xC, event.state.is_pressed()); }
                            19 => { self.chip8.set_key(0xD, event.state.is_pressed()); }
                            33 => { self.chip8.set_key(0xE, event.state.is_pressed()); }
                            47 => { self.chip8.set_key(0xF, event.state.is_pressed()); }
                            _ => {}
                        }
                    }
                    None => {}
                }
            },
            _ => (),
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        self.handle_cpu();
        if self.chip8.has_drawn() { self.draw_next_frame = true; }
        if self.draw_next_frame { self.handle_display(); }
    }
}

pub fn run(chip8: chip8::Chip8) {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app: App = App::new(chip8);
    let _ = event_loop.run_app(&mut app);
}
