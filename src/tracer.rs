use crate::camera::Camera;
use crate::geometry::point_light::PointLight;
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

        if let None = closest_intersect {
            let x = x / 2.0 + 0.5;
            let y = y / 2.0 + 0.5;
            return (x % 1.0, y % 1.0, (x + y) % 1.0);
        }

        let (sph, t) = closest_intersect.unwrap();

        let ip = camera.pos + dir * t;
        let diff_color = Tracer::trace_to_lights(ip, sph.normal(ip), &scene.lights);
        //let c = 1.0 - t * t / (sph.center - camera.pos).len_sq();
        (
            diff_color.x * sph.color.x,
            diff_color.y * sph.color.y,
            diff_color.z * sph.color.z,
        )
    }

    fn trace_to_lights(ip: Vector, normal: Vector, lights: &Vec<PointLight>) -> Vector {
        let mut total_color = Vector::zero();

        for light in lights {
            let to_light = light.pos - ip;
            let diffuse_coeff = to_light.dot(&normal);
            if diffuse_coeff <= 0.0 {
                continue;
            }
            total_color += light.color * diffuse_coeff;
        }

        total_color
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
