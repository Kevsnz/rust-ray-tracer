use crate::camera::Camera;
use crate::geometry::scene::Scene;
use crate::geometry::sphere::Sphere;
use crate::geometry::vector::Vector;

pub struct Tracer {}

impl Tracer {
    pub fn new() -> Tracer {
        Tracer {}
    }

    pub fn trace(&self, x: f64, y: f64, camera: &Camera, scene: &Scene) -> Vector {
        let vp_h = camera.up * camera.vfov2_tg;
        let vp_w = camera.rgt * camera.vfov2_tg * camera.ar;
        let dir = (camera.fwd + x * vp_w + y * vp_h).normalized();

        let closest_intersect = Self::closest_intersect(camera.pos, dir, &scene.spheres);

        if let None = closest_intersect {
            return Vector::zero();
        }

        let (sph, t) = closest_intersect.unwrap();

        let ip = camera.pos + dir * t;
        let diff_color = Tracer::trace_to_lights(ip, sph.normal(ip), scene);

        scene.ambient_light + diff_color.scale(&sph.color)
    }

    fn trace_to_lights(ip: Vector, normal: Vector, scene: &Scene) -> Vector {
        let mut total_color = Vector::zero();

        for light in &scene.point_lights {
            let to_light = light.pos - ip;
            let dist_to_light_sq = to_light.len_sq();
            let to_light = to_light.normalized();

            let diffuse_coeff = to_light.dot(&normal);
            if diffuse_coeff <= 0.0 {
                // light is on opposite side - skip it
                continue;
            }

            let ip2 = Tracer::closest_intersect(ip, to_light, &scene.spheres);
            if let Some((_, ip2t)) = ip2 {
                if ip2t * ip2t < dist_to_light_sq {
                    // path to light is occluded by geometry
                    continue;
                }
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
