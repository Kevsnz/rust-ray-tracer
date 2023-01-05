mod camera;
mod geometry;
mod renderer;
mod tracer;

use sdl2::{event::Event, keyboard::Keycode};
use std::f64::consts::PI;

use camera::Camera;
use geometry::vector::Vector;
use renderer::Renderer;
use tracer::Tracer;

const SCR_W: u32 = 1280;
const SCR_H: u32 = 960;

const REAL_W: u32 = 640;
const REAL_H: u32 = 480;

const VFOV: f64 = PI / 3.0;

fn main() {
    let camera = Camera::new(
        Vector::new(0.0, 0.0, 0.0),
        Vector::new(0.0, 0.0, 1.0),
        Vector::new(0.0, 1.0, 0.0),
        VFOV,
        REAL_W as f64 / REAL_H as f64,
    );
    let tracer = Tracer::new();

    let renderer = Renderer::initialize(SCR_W, SCR_H, REAL_W, REAL_H);
    run_render_loop(renderer, tracer, camera);
}

fn run_render_loop(mut renderer: Renderer, tracer: Tracer, camera: Camera) {
    loop {
        if handle_events(&mut renderer.event_pump) {
            break;
        }

        renderer.draw_frame(&tracer, &camera);
    }
}

pub fn handle_events(event_pump: &mut sdl2::EventPump) -> bool {
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => return true,
            _ => {}
        }
    }
    false
}
