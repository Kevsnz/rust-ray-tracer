use crate::camera::Camera;
use crate::geometry::scene::Scene;
use crate::geometry::sphere::Sphere;
use crate::geometry::vector::Vector;

pub struct Tracer {}

impl Tracer {
    pub fn new() -> Tracer {
        Tracer {}
    }

    pub fn trace(&self, x: f64, y: f64, camera: &Camera, scene: &Scene) -> (f64, f64, f64) {
        let vp_h = camera.up * camera.vfov2_tg;
        let vp_w = camera.rgt * camera.vfov2_tg * camera.ar;
        let dir = (camera.fwd + x * vp_w + y * vp_h).normalized();

        let closest_intersect = Self::closest_intersect(camera.pos, dir, &scene.spheres);

        if let Some((sph, t)) = closest_intersect {
            let c = 1.0 - t * t / (sph.center - camera.pos).len_sq();
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
