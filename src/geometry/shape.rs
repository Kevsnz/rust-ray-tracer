use super::vector::Vector;

pub trait Shape {
    fn intersect(&self, source: Vector, direction: Vector) -> Option<f64>;
    fn normal(&self, intersect_point: Vector) -> Vector;
    fn get_color(&self) -> &Vector;
}
