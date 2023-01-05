use crate::geometry::sphere::Sphere;
use crate::geometry::vector::Vector;

pub struct Tracer {
    spheres: Vec<Sphere>,
    pos: Vector,
    fwd: Vector,
    up: Vector,
    rgt: Vector,
    vfov2_tg: f64,
    ar: f64,
}

impl Tracer {
    pub fn new(ar: f64, vfov: f64) -> Tracer {
        let pos = Vector::new(0.0, 0.0, 0.0);
        let fwd = Vector::new(0.0, 0.0, 1.0);
        let rgt = Vector::new(1.0, 0.0, 0.0);
        let up = fwd.cross(&rgt);

        Tracer {
            spheres: vec![
                Sphere::new(Vector::new(0.0, 0.0, 3.0), 1.0),
                Sphere::new(Vector::new(1.0, 1.0, 4.0), 0.75),
            ],
            pos,
            fwd,
            up,
            rgt,
            vfov2_tg: (vfov / 2.0).tan(),
            ar,
        }
    }

    pub fn trace(&self, x: f64, y: f64) -> (f64, f64, f64) {
        let vp_h = self.up * self.vfov2_tg;
        let vp_w = self.rgt * self.vfov2_tg * self.ar;
        let dir = (self.fwd + x * vp_w + y * vp_h).normalized();

        let closest_intersect = Self::closest_intersect(self.pos, dir, &self.spheres);

        if let Some((sph, t)) = closest_intersect {
            let c = 1.0 - t * t / (sph.center - self.pos).len_sq();
            return (c, c, c);
        }

        let x = x / 2.0 + 0.5;
        let y = y / 2.0 + 0.5;
        (x % 1.0, y % 1.0, (x + y) % 1.0)
    }

    fn closest_intersect(
        pos: Vector,
        dir: Vector,
        spheres: &Vec<Sphere>,
    ) -> Option<(&Sphere, f64)> {
        let mut closest_intersect = None;

        for sph in spheres {
            let t = sph.intersect(pos, dir);
            if let None = t {
                continue;
            }
            if let Some((_, x)) = closest_intersect {
                if t.unwrap() >= x {
                    continue;
                }
            }
            closest_intersect = Some((sph, t.unwrap()));
        }

        closest_intersect
    }
}
