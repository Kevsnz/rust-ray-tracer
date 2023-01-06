use crate::geometry::vector::Vector;

pub struct Camera {
    pub pos: Vector,
    pub forward: Vector,
    pub up: Vector,
    pub right: Vector,
    pub vfov: f64,
    pub ar: f64,
    pub vfov2_tg: f64,
}

impl Camera {
    pub fn new(pos: Vector, dir: Vector, up: Vector, vfov: f64, ar: f64) -> Camera {
        let dir = dir.normalized();
        let rgt = up.cross(&dir).normalized();
        let up = dir.cross(&rgt);

        Camera {
            pos,
            forward: dir,
            up,
            right: rgt,
            vfov,
            ar,
            vfov2_tg: (vfov / 2.0).tan(),
        }
    }

    pub fn shift_vertical(&mut self, up: f64) {
        self.pos += self.up * up;
    }
    pub fn shift_lateral(&mut self, right: f64) {
        self.pos += self.right * right;
    }
    pub fn shift_longitudinal(&mut self, forward: f64) {
        self.pos += self.forward * forward;
    }

    pub fn rotate_pitch(&mut self, pitch_down: f64) {
        self.forward = self.forward.rotate(&self.right, pitch_down);
        self.up = self.forward.cross(&self.right);
    }
    pub fn rotate_yaw(&mut self, yaw_right: f64) {
        self.forward = self.forward.rotate(&self.up, yaw_right);
        self.right = self.up.cross(&self.forward);
    }
    pub fn rotate_roll(&mut self, roll_left: f64) {
        self.up = self.up.rotate(&self.forward, roll_left);
        self.right = self.up.cross(&self.forward);
    }
}
