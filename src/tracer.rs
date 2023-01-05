pub struct Tracer {
    width: u32,
    height: u32,
}

impl Tracer {
    pub fn new(w: u32, h: u32) -> Tracer {
        Tracer {
            width: w,
            height: h,
        }
    }

    pub fn trace(&self, x: u32, y: u32) -> (f64, f64, f64) {
        let x = x as f64 / self.width as f64;
        let y = y as f64 / self.height as f64;

        (x % 1.0, y % 1.0, (x + y) % 1.0)
    }
}
