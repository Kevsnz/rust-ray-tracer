use super::{point_light::PointLight, sphere::Sphere, vector::Vector};

pub struct Scene {
    pub spheres: Vec<Sphere>,
    pub lights: Vec<PointLight>,
}

impl Scene {
    pub fn new() -> Scene {
        let spheres = vec![
            Sphere::new(Vector::new(0.0, 0.0, 3.0), 1.0, Vector::new(1.0, 0.5, 0.0)),
            Sphere::new(Vector::new(1.0, 1.0, 4.0), 0.75, Vector::new(0.0, 0.0, 1.0)),
        ];

        let lights = vec![
            PointLight::new(Vector::new(2.0, 3.0, 2.0), Vector::new(0.25, 0.5, 0.75)),
            PointLight::new(Vector::new(-1.0, 0.0, 1.0), Vector::new(0.75, 0.0, 0.25)),
        ];

        Scene { spheres, lights }
    }
}
