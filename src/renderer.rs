use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::Canvas;

use crate::tracer::Tracer;

pub struct Renderer {
    canvas: Canvas<sdl2::video::Window>,
    event_pump: sdl2::EventPump,
    tracer: Tracer,
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

        let tracer = Tracer::new(render_width, render_height);

        Renderer {
            canvas,
            event_pump,
            tracer,
        }
    }

    pub fn draw_frame(&mut self) {
        let (w, h) = self.canvas.logical_size();
        self.canvas.set_draw_color(Color::RGB(0, 8, 32));
        self.canvas.clear();

        let tex_creator = self.canvas.texture_creator();
        let mut tex = tex_creator
            .create_texture_streaming(None, w, h)
            .expect("Cannot create texture for rendering!");

        tex.with_lock(None, |buf, stride| self.render(buf, stride))
            .expect("Cannot render frame into texture!");

        self.canvas
            .copy(&tex, None, None)
            .expect("Cannot copy texture to framebuffer!");
        self.canvas.present();
    }

    fn render(&self, buf: &mut [u8], stride: usize) {
        let (w, h) = self.canvas.logical_size();

        for y in 0..h as u32 {
            for x in 0..w as u32 {
                let pos = (y * stride as u32 + x * 4) as usize;
                let (r, g, b) = self.tracer.trace(x, y);

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

    pub fn handle_events(&mut self) -> bool {
        for event in self.event_pump.poll_iter() {
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
}
