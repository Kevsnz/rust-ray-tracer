use crate::camera::Camera;
use crate::geometry::scene::Scene;
use crate::geometry::shape::Shape;
use crate::geometry::vector::Vector;

pub struct Tracer {}

impl Tracer {
    pub fn new() -> Tracer {
        Tracer {}
    }

    pub fn trace(&self, x: f64, y: f64, camera: &Camera, scene: &Scene, refl_idx: i32) -> Vector {
        let vp_h = camera.up * camera.vfov2_tg;
        let vp_w = camera.right * camera.vfov2_tg * camera.ar;
        let dir = (camera.forward + x * vp_w + y * vp_h).normalized();

        Tracer::trace_color(camera.pos, dir, scene, refl_idx)
    }

    fn trace_color(source: Vector, direction: Vector, scene: &Scene, refl_idx: i32) -> Vector {
        let closest_intersect = Self::closest_intersect(source, direction, &scene.shapes);

        if let None = closest_intersect {
            return Vector::zero();
        }

        let mut result_color = scene.ambient_light.clone();

        let (shape, t) = closest_intersect.unwrap();
        let material = shape.get_material();

        let ip = source + direction * t;
        let normal = shape.normal(ip);
        let diff_color = Self::trace_to_lights(ip, normal, scene);
        result_color += diff_color.scale(&material.color) * (1.0 - material.refletivity_index);

        if refl_idx > 0 {
            let refl_direction = direction.reflect(&normal);
            let refl_color = Self::trace_color(ip, refl_direction, scene, refl_idx - 1);
            result_color += refl_color.scale(&material.color) * material.refletivity_index;
        }

        result_color
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

            let ip2 = Self::closest_intersect(ip, to_light, &scene.shapes);
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
        shapes: &Vec<Box<dyn Shape>>,
    ) -> Option<(&dyn Shape, f64)> {
        let mut closest_intersect = None;

        for sph in shapes {
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
