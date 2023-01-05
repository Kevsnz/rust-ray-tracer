use crate::geometry::vector::Vector;

pub struct Camera {
    pub pos: Vector,
    pub fwd: Vector,
    pub up: Vector,
    pub rgt: Vector,
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
            fwd: dir,
            up,
            rgt,
            vfov,
            ar,
            vfov2_tg: (vfov / 2.0).tan(),
        }
    }
}
