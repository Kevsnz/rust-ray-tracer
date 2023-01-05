use super::{sphere::Sphere, vector::Vector};

pub struct Scene {
    pub spheres: Vec<Sphere>,
}

impl Scene {
    pub fn new() -> Scene {
        let spheres = vec![
            Sphere::new(Vector::new(0.0, 0.0, 3.0), 1.0),
            Sphere::new(Vector::new(1.0, 1.0, 4.0), 0.75),
        ];

        Scene { spheres }
    }
}
