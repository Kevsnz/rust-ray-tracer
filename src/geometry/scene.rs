use crate::material::Material;

use super::{
    planes::{PlaneXY, PlaneXZ, PlaneYZ},
    point_light::PointLight,
    shape::Shape,
    sphere::Sphere,
    vector::Vector,
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
                Material {
                    color: Vector::new(1.0, 0.9, 0.6),
                    refletivity_index: 0.35,
                },
            )),
            Box::new(Sphere::new(
                Vector::new(1.0, 1.0, 4.0),
                0.75,
                Material {
                    color: Vector::new(0.7, 0.7, 1.0),
                    refletivity_index: 0.15,
                },
            )),
            Box::new(PlaneXY::new(
                5.0,
                true,
                -3.0,
                -1.0,
                3.0,
                3.0,
                Material {
                    color: Vector::new(0.5, 1.0, 0.8),
                    refletivity_index: 0.75,
                },
            )),
            Box::new(PlaneXZ::new(
                -1.0,
                false,
                -3.0,
                1.0,
                3.0,
                5.0,
                Material {
                    color: Vector::new(1.0, 0.25, 0.0),
                    refletivity_index: 0.5,
                },
            )),
            Box::new(PlaneYZ::new(
                3.0,
                true,
                -1.0,
                1.0,
                3.0,
                5.0,
                Material {
                    color: Vector::new(0.7, 0.25, 1.0),
                    refletivity_index: 0.6,
                },
            )),
        ];

        let point_lights = vec![
            PointLight::new(Vector::new(2.0, 3.0, 2.0), Vector::new(0.5, 0.6, 0.75), 3.0),
            PointLight::new(
                Vector::new(-2.0, 0.0, 0.0),
                Vector::new(0.5, 0.3, 0.45),
                4.5,
            ),
            PointLight::new(
                Vector::new(-4.0, 0.0, 3.0),
                Vector::new(1.0, 0.95, 0.9),
                3.5,
            ),
        ];

        Scene {
            shapes,
            ambient_light: Vector::new(0.01, 0.02, 0.04),
            point_lights,
        }
    }
}
