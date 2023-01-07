use super::{shape::Shape, vector::Vector};

pub struct PlaneXY {
    pub z: f64,
    pub negative: bool,
    pub x_min: f64,
    pub y_min: f64,
    pub x_max: f64,
    pub y_max: f64,
    pub color: Vector,
}

impl PlaneXY {
    pub fn new(
        z: f64,
        negative: bool,
        x_min: f64,
        y_min: f64,
        x_max: f64,
        y_max: f64,
        color: Vector,
    ) -> PlaneXY {
        PlaneXY {
            z,
            negative,
            x_min,
            y_min,
            x_max,
            y_max,
            color,
        }
    }
}

impl Shape for PlaneXY {
    fn intersect(
        &self,
        source: super::vector::Vector,
        direction: super::vector::Vector,
    ) -> Option<f64> {
        // x.z = s.z + t*d.z
        // x.z = p.z
        // t = (x.z - s.z) / d.z
        // t = (p.z - s.z) / d.z
        if direction.z == 0.0 {
            return None;
        }

        if (direction.z < 0.0) == self.negative {
            return None;
        }

        let t = (self.z - source.z) / direction.z;
        if t < 0.0 {
            return None;
        }

        let ip = source + direction * t;
        if ip.x < self.x_min || ip.x > self.x_max || ip.y < self.y_min || ip.y > self.y_max {
            return None;
        }
        Some(t)
    }

    fn normal(&self, _: Vector) -> Vector {
        if self.negative {
            Vector::new(0.0, 0.0, -1.0)
        } else {
            Vector::new(0.0, 0.0, 1.0)
        }
    }

    fn get_color(&self) -> &Vector {
        &self.color
    }
}
