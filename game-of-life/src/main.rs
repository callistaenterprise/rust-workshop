#![deny(clippy::all)]
#![forbid(unsafe_code)]

use std::time::{Duration, SystemTime};

use game_of_life_ws::World;
use log::error;
use pixels::{Error, Pixels, SurfaceTexture};
use winit::{
    dpi::LogicalSize,
    event::{Event, VirtualKeyCode},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use winit_input_helper::WinitInputHelper;

const DIM: u32 = 300;
const FPS: u32 = 60;

fn main() -> Result<(), Error> {
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();

    let mut world = World::new(DIM as usize);

    let window = {
        let size = LogicalSize::new(DIM as f64, DIM as f64);
        let scaled_size = LogicalSize::new(DIM as f64 * 3.0, DIM as f64 * 3.0);
        WindowBuilder::new()
            .with_title("Game of Life (P: pause, Space: step)")
            .with_inner_size(scaled_size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(DIM, DIM, surface_texture)?
    };

    let mut paused = true;
    let mut t = SystemTime::now();
    event_loop.run(move |event, _, control_flow| {
        if let Event::RedrawRequested(_) = event {
            let screen = pixels.get_frame_mut();

            world.draw(screen);

            if pixels
                .render()
                .map_err(|e| error!("pixels.render() failed: {}", e))
                .is_err()
            {
                *control_flow = ControlFlow::Exit;
                return;
            }
        }
        if input.update(&event) {
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }
            if input.key_pressed(VirtualKeyCode::P) {
                paused = !paused;
            }
            if input.key_pressed(VirtualKeyCode::Space) {
                paused = true;
                world.update();
            }

            // Resize the window
            if let Some(size) = input.window_resized() {
                pixels.resize_surface(size.width, size.height);
            }

            if SystemTime::now().duration_since(t).unwrap() > Duration::new(0, 1_000_000_000 / FPS)
            {
                t = SystemTime::now();
                if !paused {
                    world.update();
                }
            }
            window.request_redraw();
        }
    });
}
