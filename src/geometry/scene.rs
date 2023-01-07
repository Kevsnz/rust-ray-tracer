use super::{
    planes::PlaneXY, point_light::PointLight, shape::Shape, sphere::Sphere, vector::Vector,
};

pub struct Scene {
    pub shapes: Vec<Box<dyn Shape>>,
    pub ambient_light: Vector,
    pub point_lights: Vec<PointLight>,
}

impl Scene {
    pub fn new() -> Scene {
        let shapes: Vec<Box<dyn Shape>> = vec![
            Box::new(Sphere::new(
                Vector::new(0.0, 0.0, 3.0),
                1.0,
                Vector::new(1.0, 0.9, 0.6),
            )),
            Box::new(Sphere::new(
                Vector::new(1.0, 1.0, 4.0),
                0.75,
                Vector::new(0.7, 0.7, 1.0),
            )),
            Box::new(PlaneXY::new(
                5.0,
                true,
                -5.0,
                -3.0,
                4.0,
                3.0,
                Vector::new(0.25, 0.75, 0.4),
            )),
        ];

        let point_lights = vec![
            PointLight::new(Vector::new(2.0, 3.0, 2.0), Vector::new(0.5, 0.6, 0.75), 2.5),
            PointLight::new(
                Vector::new(-2.0, 0.0, 0.0),
                Vector::new(0.5, 0.3, 0.45),
                4.0,
            ),
        ];

        Scene {
            shapes,
            ambient_light: Vector::new(0.01, 0.02, 0.04),
            point_lights,
        }
    }
}
