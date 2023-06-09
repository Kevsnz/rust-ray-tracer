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
        let sphere_big = Sphere::new(
            Vector::new(0.0, -0.5, 3.0),
            1.25,
            Material {
                color: Vector::new(1.0, 0.8, 0.5),
                refletivity_index: 0.2,
            },
        );
        let sphere_small = Sphere::new(
            Vector::new(1.0, 1.0, 4.0),
            0.75,
            Material {
                color: Vector::new(0.7, 0.7, 1.0),
                refletivity_index: 0.10,
            },
        );

        let plane_front = PlaneXY::new(
            7.0,
            true,
            (-3.0, 4.0),
            (-2.0, 3.0),
            Material {
                color: Vector::new(0.5, 1.0, 0.8),
                refletivity_index: 0.75,
            },
        );
        let plane_bottom = PlaneXZ::new(
            -2.0,
            false,
            (-3.0, 4.0),
            (1.0, 7.0),
            Material {
                color: Vector::new(1.0, 0.25, 0.0),
                refletivity_index: 0.65,
            },
        );
        let plane_right = PlaneYZ::new(
            4.0,
            true,
            (-2.0, 3.0),
            (1.0, 7.0),
            Material {
                color: Vector::new(0.7, 0.25, 1.0),
                refletivity_index: 0.8,
            },
        );
        let shapes: Vec<Box<dyn Shape>> = vec![
            Box::new(sphere_big),
            Box::new(sphere_small),
            Box::new(plane_front),
            Box::new(plane_bottom),
            Box::new(plane_right),
        ];

        let point_lights = vec![
            PointLight::new(Vector::new(2.5, 4.0, 2.0), Vector::new(0.7, 0.8, 1.0), 5.0),
            PointLight::new(
                Vector::new(-5.0, 2.0, -3.0),
                Vector::new(0.8, 1.0, 0.95),
                9.0,
            ),
            PointLight::new(
                Vector::new(-4.0, 0.0, 3.0),
                Vector::new(1.0, 0.95, 0.9),
                3.0,
            ),
        ];

        Scene {
            shapes,
            ambient_light: Vector::new(0.01, 0.02, 0.04),
            point_lights,
        }
    }
}
