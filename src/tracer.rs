use crate::camera::Camera;
use crate::geometry::scene::Scene;
use crate::geometry::shape::Shape;
use crate::geometry::vector::Vector;

pub struct Tracer {}

impl Tracer {
    pub fn new() -> Tracer {
        Tracer {}
    }

    pub fn trace(&self, x: f64, y: f64, camera: &Camera, scene: &Scene) -> Vector {
        let vp_h = camera.up * camera.vfov2_tg;
        let vp_w = camera.right * camera.vfov2_tg * camera.ar;
        let dir = (camera.forward + x * vp_w + y * vp_h).normalized();

        let closest_intersect = Self::closest_intersect(camera.pos, dir, &scene.shapes);

        if let None = closest_intersect {
            return Vector::zero();
        }

        let (sph, t) = closest_intersect.unwrap();

        let ip = camera.pos + dir * t;
        let diff_color = Tracer::trace_to_lights(ip, sph.normal(ip), scene);

        scene.ambient_light + diff_color.scale(&sph.get_color())
    }

    fn trace_to_lights(ip: Vector, normal: Vector, scene: &Scene) -> Vector {
        let mut total_color = Vector::zero();

        for light in &scene.point_lights {
            let to_light = light.pos - ip;
            let dist_to_light_sq = to_light.len_sq();
            let to_light = to_light.normalized();

            let incidence_coeff = to_light.dot(&normal);
            if incidence_coeff <= 0.0 {
                // light is on opposite side - skip it
                continue;
            }

            let ip2 = Tracer::closest_intersect(ip, to_light, &scene.shapes);
            if let Some((_, ip2t)) = ip2 {
                if ip2t.powi(2) < dist_to_light_sq {
                    // path to light is occluded by geometry
                    continue;
                }
            }

            total_color += light.color * (light.power.powi(2) / dist_to_light_sq) * incidence_coeff;
        }

        total_color
    }

    fn closest_intersect(
        pos: Vector,
        dir: Vector,
        spheres: &Vec<Box<dyn Shape>>,
    ) -> Option<(&dyn Shape, f64)> {
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
            closest_intersect = Some((sph.as_ref(), t.unwrap()));
        }

        closest_intersect
    }
}
