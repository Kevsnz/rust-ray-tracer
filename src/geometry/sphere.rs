use super::shape::Shape;
use crate::{geometry::vector::Vector, material::Material};

pub struct Sphere {
    pub center: Vector,
    pub radius: f64,
    pub material: Material,
}

impl Sphere {
    pub fn new(pos: Vector, radius: f64, color: Material) -> Sphere {
        Sphere {
            center: pos,
            radius,
            material: color,
        }
    }
}

impl Shape for Sphere {
    #[inline]
    fn normal(&self, t: Vector) -> Vector {
        (t - self.center) / self.radius
    }

    fn intersect(&self, src: Vector, dir: Vector) -> Option<f64> {
        // Solves system of equations w.r.t. t (intersect distance from ray source):
        // x = src + t * dir
        // |x - c|^2 = r^2

        let v = src - self.center;
        let vd = v.dot(&dir);

        let dd = vd * vd - (v.len_sq() - self.radius * self.radius);
        if dd < 0.0 {
            return None;
        }
        let t = -vd - dd.sqrt();
        if t < 0.0 {
            return None;
        }
        Some(t)
    }

    #[inline]
    fn get_material(&self) -> &Material {
        &self.material
    }
}

#[cfg(test)]
mod tests {
    use crate::geometry::shape::Shape;
    use crate::geometry::vector::Vector;
    use crate::material::Material;

    use super::Sphere;

    #[test]
    fn create() {
        let s = Sphere::new(
            Vector::zero(),
            1.0,
            Material {
                color: Vector::zero(),
                refletivity_index: 0.0,
            },
        );
        assert_eq!(s.center, Vector::zero());
        assert_eq!(s.radius, 1.0);
        assert_eq!(s.material.color, Vector::zero());
    }

    #[test]
    fn intersect_some() {
        let s = Sphere::new(
            Vector::zero(),
            1.0,
            Material {
                color: Vector::zero(),
                refletivity_index: 0.0,
            },
        );
        let i = s.intersect(Vector::new(2.0, 0.0, 0.0), Vector::new(-1.0, 0.0, 0.0));
        assert_eq!(i, Some(1.0));
    }

    #[test]
    fn intersect_none() {
        let s = Sphere::new(
            Vector::zero(),
            1.0,
            Material {
                color: Vector::zero(),
                refletivity_index: 0.0,
            },
        );
        let i = s.intersect(Vector::new(2.0, 0.0, 0.0), Vector::new(1.0, 0.0, 0.0));
        assert_eq!(i, None);
    }

    #[test]
    fn normal() {
        let s = Sphere::new(
            Vector::zero(),
            1.0,
            Material {
                color: Vector::zero(),
                refletivity_index: 0.0,
            },
        );
        assert_eq!(s.normal(Vector::one_x()), Vector::one_x());
        assert_eq!(s.normal(Vector::one_y()), Vector::one_y());
        assert_eq!(s.normal(Vector::one_z()), Vector::one_z());
    }
}
