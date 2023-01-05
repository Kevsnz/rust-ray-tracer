use sdl2::pixels::Color;
use sdl2::render::Canvas;

use crate::camera::Camera;
use crate::geometry::scene::Scene;
use crate::tracer::Tracer;

pub struct Renderer {
    canvas: Canvas<sdl2::video::Window>,
    pub event_pump: sdl2::EventPump,
}

impl Renderer {
    pub fn initialize(
        window_width: u32,
        window_height: u32,
        render_width: u32,
        render_height: u32,
    ) -> Renderer {
        let sdl_context = sdl2::init().unwrap_or_else(|err| {
            println!("Cannot initialize SDL! {}", err);
            std::process::exit(1);
        });

        let video_subsystem = sdl_context
            .video()
            .expect("Cannot initialize video for SDL!");

        let mut window = video_subsystem
            .window("ray tracer", window_width, window_height)
            .resizable()
            .position_centered()
            .build()
            .expect("Cannot initialize video mode for SDL! {}");

        window
            .set_title("Ray Tracer")
            .expect("Failed to set window title!");

        let event_pump = sdl_context
            .event_pump()
            .expect("Cannot initialize event pump for SDL!");

        let mut canvas = window
            .into_canvas()
            .present_vsync()
            .build()
            .expect("Failed to get canvas from window!");

        canvas
            .set_integer_scale(true)
            .expect("Failed to set integer scale!");
        canvas
            .set_logical_size(render_width, render_height)
            .expect("Failed to set logical size for the canvas!");

        Renderer { canvas, event_pump }
    }

    pub fn draw_frame(&mut self, tracer: &Tracer, camera: &Camera, scene: &Scene) {
        let (w, h) = self.canvas.logical_size();
        self.canvas.set_draw_color(Color::RGB(0, 8, 32));
        self.canvas.clear();

        let tex_creator = self.canvas.texture_creator();
        let mut tex = tex_creator
            .create_texture_streaming(None, w, h)
            .expect("Cannot create texture for rendering!");

        tex.with_lock(None, |buf, stride| {
            self.render(buf, stride, tracer, camera, scene)
        })
        .expect("Cannot render frame into texture!");

        self.canvas
            .copy(&tex, None, None)
            .expect("Cannot copy texture to framebuffer!");
        self.canvas.present();
    }

    fn render(
        &self,
        buf: &mut [u8],
        stride: usize,
        tracer: &Tracer,
        camera: &Camera,
        scene: &Scene,
    ) {
        let (w, h) = self.canvas.logical_size();

        for y in 0..h as u32 {
            for x in 0..w as u32 {
                let pos = (y * stride as u32 + x * 4) as usize;
                let xp = (x as f64 + 0.5) / (w as f64 / 2.0) - 1.0;
                let yp = (y as f64 + 0.5) / (h as f64 / 2.0) - 1.0;
                let (r, g, b) = tracer.trace(xp, -yp, camera, scene); // vertical axis is inverted on screen

                let r = (r * 255.9) as u8;
                let g = (g * 255.9) as u8;
                let b = (b * 255.9) as u8;

                buf[pos + 0] = b; // b
                buf[pos + 1] = g; // g
                buf[pos + 2] = r; // r
                buf[pos + 3] = 255; // a?
            }
        }
    }
}
