use super::vector::Vector;

pub struct PointLight {
    pub pos: Vector,
    pub color: Vector,
}

impl PointLight {
    #[inline]
    pub fn new(pos: Vector, color: Vector) -> PointLight {
        PointLight { pos, color }
    }
}
