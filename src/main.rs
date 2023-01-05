mod geometry;
mod renderer;
mod tracer;
use renderer::Renderer;

const SCR_W: u32 = 1280;
const SCR_H: u32 = 960;

const REAL_W: u32 = 640;
const REAL_H: u32 = 480;

fn main() {
    let tracer = Renderer::initialize(SCR_W, SCR_H, REAL_W, REAL_H);
    run_render_loop(tracer);
}

fn run_render_loop(mut tracer: Renderer) {
    loop {
        if tracer.handle_events() {
            break;
        }

        tracer.draw_frame();
    }
}
