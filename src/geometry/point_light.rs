use super::vector::Vector;

pub struct PointLight {
    pub pos: Vector,
    pub color: Vector,
    pub power: f64,
}

impl PointLight {
    #[inline]
    pub fn new(pos: Vector, color: Vector, power: f64) -> PointLight {
        PointLight { pos, color, power }
    }
}
