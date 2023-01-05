use crate::geometry::vector::Vector;

pub struct Sphere {
    pub center: Vector,
    pub radius: f64,
    pub color: Vector,
}

impl Sphere {
    pub fn new(pos: Vector, radius: f64, color: Vector) -> Sphere {
        Sphere {
            center: pos,
            radius,
            color,
        }
    }
    pub fn intersect(&self, src: Vector, dir: Vector) -> Option<f64> {
        // Solves system of equations w.r.t.  t (intersect distance from ray source):
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
}

#[cfg(test)]
mod tests {
    use crate::geometry::vector::Vector;

    use super::Sphere;

    #[test]
    fn create() {
        let s = Sphere::new(Vector::zero(), 1.0, Vector::zero());
        assert_eq!(s.center, Vector::zero());
        assert_eq!(s.radius, 1.0);
        assert_eq!(s.color, Vector::zero());
    }

    #[test]
    fn intersect_some() {
        let s = Sphere::new(Vector::zero(), 1.0, Vector::zero());
        let i = s.intersect(Vector::new(2.0, 0.0, 0.0), Vector::new(-1.0, 0.0, 0.0));
        assert_eq!(i, Some(1.0));
    }

    #[test]
    fn intersect_none() {
        let s = Sphere::new(Vector::zero(), 1.0, Vector::zero());
        let i = s.intersect(Vector::new(2.0, 0.0, 0.0), Vector::new(1.0, 0.0, 0.0));
        assert_eq!(i, None);
    }
}
