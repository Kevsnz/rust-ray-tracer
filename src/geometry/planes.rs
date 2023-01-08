use crate::material::Material;

use super::{shape::Shape, vector::Vector};

pub struct Plane {
    pub fixed: f64,
    pub negative: bool,
    pub x_range: (f64, f64),
    pub y_range: (f64, f64),
    pub material: Material,
}

impl Plane {
    pub fn new(
        fixed: f64,
        negative: bool,
        x_range: (f64, f64),
        y_range: (f64, f64),
        material: Material,
    ) -> Plane {
        Plane {
            fixed,
            negative,
            x_range,
            y_range,
            material,
        }
    }

    pub fn intersect(
        &self,
        source: super::vector::Vector,
        direction: super::vector::Vector,
    ) -> Option<f64> {
        // Solves system of equations w.r.t. t (intersect distance from ray source):
        // x.z = src.z + t * dir.z
        // x.z = p.z
        if direction.z == 0.0 {
            return None;
        }

        if (direction.z < 0.0) == self.negative {
            return None;
        }

        let t = (self.fixed - source.z) / direction.z;
        if t < 0.0 {
            return None;
        }

        let ip_x = source.x + direction.x * t;
        let ip_y = source.y + direction.y * t;
        if ip_x < self.x_range.0
            || ip_x > self.x_range.1
            || ip_y < self.y_range.0
            || ip_y > self.y_range.1
        {
            return None;
        }
        Some(t)
    }
}

pub struct PlaneXZ {
    plane: Plane,
}

impl PlaneXZ {
    pub fn new(
        y: f64,
        negative: bool,
        x_range: (f64, f64),
        z_range: (f64, f64),
        material: Material,
    ) -> PlaneXZ {
        PlaneXZ {
            plane: Plane::new(y, negative, x_range, z_range, material),
        }
    }
}

impl Shape for PlaneXZ {
    fn intersect(
        &self,
        source: super::vector::Vector,
        direction: super::vector::Vector,
    ) -> Option<f64> {
        let src = Vector::new(source.x, source.z, source.y);
        let dir = Vector::new(direction.x, direction.z, direction.y);
        self.plane.intersect(src, dir)
    }

    #[inline]
    fn normal(&self, _: Vector) -> Vector {
        Vector::new(0.0, if self.plane.negative { -1.0 } else { 1.0 }, 0.0)
    }

    #[inline]
    fn get_material(&self) -> &Material {
        &self.plane.material
    }
}

pub struct PlaneXY {
    pub plane: Plane,
}

impl PlaneXY {
    pub fn new(
        z: f64,
        negative: bool,
        x_range: (f64, f64),
        y_range: (f64, f64),
        material: Material,
    ) -> PlaneXY {
        PlaneXY {
            plane: Plane::new(z, negative, x_range, y_range, material),
        }
    }
}

impl Shape for PlaneXY {
    fn intersect(
        &self,
        source: super::vector::Vector,
        direction: super::vector::Vector,
    ) -> Option<f64> {
        self.plane.intersect(source, direction)
    }

    #[inline]
    fn normal(&self, _: Vector) -> Vector {
        Vector::new(0.0, 0.0, if self.plane.negative { -1.0 } else { 1.0 })
    }

    #[inline]
    fn get_material(&self) -> &Material {
        &self.plane.material
    }
}

pub struct PlaneYZ {
    pub plane: Plane,
}

impl PlaneYZ {
    pub fn new(
        x: f64,
        negative: bool,
        y_range: (f64, f64),
        z_range: (f64, f64),
        material: Material,
    ) -> PlaneYZ {
        PlaneYZ {
            plane: Plane::new(x, negative, y_range, z_range, material),
        }
    }
}

impl Shape for PlaneYZ {
    fn intersect(
        &self,
        source: super::vector::Vector,
        direction: super::vector::Vector,
    ) -> Option<f64> {
        let src = Vector::new(source.y, source.z, source.x);
        let dir = Vector::new(direction.y, direction.z, direction.x);
        self.plane.intersect(src, dir)
    }

    #[inline]
    fn normal(&self, _: Vector) -> Vector {
        Vector::new(if self.plane.negative { -1.0 } else { 1.0 }, 0.0, 0.0)
    }

    #[inline]
    fn get_material(&self) -> &Material {
        &self.plane.material
    }
}
