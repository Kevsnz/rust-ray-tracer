use std::ops::{Add, AddAssign, Div, Mul, Neg, Sub};

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector {
    #[inline]
    pub fn new(x: f64, y: f64, z: f64) -> Vector {
        Vector { x, y, z }
    }

    #[inline]
    pub fn zero() -> Vector {
        Vector {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    #[inline]
    pub fn one_x() -> Vector {
        Vector {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        }
    }

    #[inline]
    pub fn one_y() -> Vector {
        Vector {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        }
    }

    #[inline]
    pub fn one_z() -> Vector {
        Vector {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        }
    }

    #[inline]
    pub fn one() -> Vector {
        Vector {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        }
    }

    #[inline]
    pub fn len(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
    #[inline]
    pub fn len_sq(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    #[inline]
    pub fn dot(&self, rhs: &Self) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
    #[inline]
    pub fn cross(&self, rhs: &Self) -> Self {
        Self::new(
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x,
        )
    }

    #[inline]
    pub fn scale(&self, rhs: &Self) -> Self {
        Self::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }

    #[inline]
    pub fn normalized(self) -> Self {
        self / self.len()
    }

    #[inline]
    pub fn spread(&self) -> (f64, f64, f64) {
        (self.x, self.y, self.z)
    }

    #[inline]
    pub fn rotate(&self, axis: &Vector, angle: f64) -> Vector {
        *axis * axis.dot(self)
            + (angle.cos() * axis.cross(self)).cross(axis)
            + angle.sin() * axis.cross(self)
    }

    #[inline]
    pub fn reflect(&self, normal: &Vector) -> Vector {
        *self - 2.0 * self.dot(normal) * *normal
    }
}

impl Add<Vector> for Vector {
    type Output = Vector;

    #[inline]
    fn add(self, rhs: Vector) -> Self::Output {
        Vector::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Sub<Vector> for Vector {
    type Output = Vector;

    #[inline]
    fn sub(self, rhs: Vector) -> Self::Output {
        Vector::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Mul<f64> for Vector {
    type Output = Vector;

    #[inline]
    fn mul(self, rhs: f64) -> Self::Output {
        Vector::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}
impl Div<f64> for Vector {
    type Output = Vector;

    #[inline]
    fn div(self, rhs: f64) -> Self::Output {
        Vector::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl Neg for Vector {
    type Output = Vector;

    #[inline]
    fn neg(self) -> Self::Output {
        Vector::new(-self.x, -self.y, -self.z)
    }
}

impl Mul<Vector> for f64 {
    type Output = Vector;

    #[inline]
    fn mul(self, rhs: Vector) -> Self::Output {
        Vector::new(rhs.x * self, rhs.y * self, rhs.z * self)
    }
}

impl AddAssign<Vector> for Vector {
    #[inline]
    fn add_assign(&mut self, rhs: Vector) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

#[cfg(test)]
mod tests {
    use crate::assert_delta;
    use std::f64::consts::PI;

    use super::Vector;

    #[test]
    fn create() {
        let v = Vector::new(1.0, 2.0, 3.0);
        assert_eq!(v.x, 1.0);
        assert_eq!(v.y, 2.0);
        assert_eq!(v.z, 3.0);
    }

    #[test]
    fn equality() {
        let v1 = Vector::new(1.0, 2.0, 3.0);
        let v2 = Vector::new(1.0, 2.0, 3.0);
        let v31 = Vector::new(1.1, 2.0, 3.0);
        let v32 = Vector::new(1.0, 2.1, 3.0);
        let v33 = Vector::new(1.0, 2.0, 3.1);

        assert!(v1 == v2);
        assert!(v1 != v31);
        assert!(v1 != v32);
        assert!(v1 != v33);
    }

    #[test]
    fn sum() {
        let v1 = Vector::new(1.0, 2.0, 3.0);
        let v2 = Vector::new(1.5, 2.5, 3.5);
        let vs = v1 + v2;
        assert_eq!(vs.x, 1.0 + 1.5);
        assert_eq!(vs.y, 2.0 + 2.5);
        assert_eq!(vs.z, 3.0 + 3.5);
    }

    #[test]
    fn sub() {
        let v1 = Vector::new(1.0, 2.0, 3.0);
        let v2 = Vector::new(1.5, 2.4, 3.3);
        let vs = v2 - v1;
        assert_eq!(vs.x, 1.5 - 1.0);
        assert_eq!(vs.y, 2.4 - 2.0);
        assert_eq!(vs.z, 3.3 - 3.0);
    }

    #[test]
    fn mul_f64() {
        let v1 = Vector::new(1.0, 2.0, 3.0);
        let vs = v1 * 2.0;
        assert_eq!(vs.x, 1.0 * 2.0);
        assert_eq!(vs.y, 2.0 * 2.0);
        assert_eq!(vs.z, 3.0 * 2.0);
    }

    #[test]
    fn div_f64() {
        let v1 = Vector::new(1.0, 2.0, 3.0);
        let vs = v1 / 2.0;
        assert_eq!(vs.x, 1.0 / 2.0);
        assert_eq!(vs.y, 2.0 / 2.0);
        assert_eq!(vs.z, 3.0 / 2.0);
    }

    #[test]
    fn negate() {
        let v = Vector::new(1.0, 2.0, 3.0);
        let vs = -v;
        assert_eq!(vs.x, -1.0);
        assert_eq!(vs.y, -2.0);
        assert_eq!(vs.z, -3.0);
    }

    #[test]
    fn add_assign() {
        let mut v = Vector::new(1.0, 2.0, 3.0);
        v += Vector::one();
        assert_eq!(v.x, 2.0);
        assert_eq!(v.y, 3.0);
        assert_eq!(v.z, 4.0);
    }

    #[test]
    fn len() {
        let v = Vector::new(1.0, 2.0, 3.0);
        assert_eq!(v.len(), (1.0 as f64 + 2.0 * 2.0 + 3.0 * 3.0).sqrt());
        assert_eq!(v.len_sq(), 1.0 as f64 + 2.0 * 2.0 + 3.0 * 3.0);
    }

    #[test]
    fn dot() {
        let v1 = Vector::new(1.0, 2.0, 3.0);
        let v2 = Vector::new(2.0, 3.0, 4.0);
        let prod = v1.dot(&v2);
        assert_eq!(prod, 1.0 * 2.0 + 2.0 * 3.0 + 3.0 * 4.0);
    }

    #[test]
    fn cross() {
        let v1 = Vector::new(1.0, 2.0, 3.0);
        let v2 = Vector::new(2.0, 3.0, 4.0);
        let vs = v1.cross(&v2);
        assert_eq!(vs.x, 2.0 * 4.0 - 3.0 * 3.0);
        assert_eq!(vs.y, 3.0 * 2.0 - 1.0 * 4.0);
        assert_eq!(vs.z, 1.0 * 3.0 - 2.0 * 2.0);
    }

    #[test]
    fn normalized() {
        let v1 = Vector::new(4.0, 6.0, 7.0);
        let v2 = v1.normalized();
        assert_eq!(v2.len(), 1.0);
        assert_eq!(v2.x, 4.0 / (101.0 as f64).sqrt());
        assert_eq!(v2.y, 6.0 / (101.0 as f64).sqrt());
        assert_eq!(v2.z, 7.0 / (101.0 as f64).sqrt());
    }

    #[test]
    fn rotate_pitch() {
        let v = Vector::new(1.0, 1.0, 1.0);
        let axis = Vector::new(1.0, 0.0, 0.0);

        compare_delta(
            v.rotate(&axis, PI / 2.0),
            Vector::new(1.0, -1.0, 1.0),
            1e-15,
        );
        compare_delta(
            v.rotate(&axis, -PI / 2.0),
            Vector::new(1.0, 1.0, -1.0),
            1e-15,
        );
    }

    #[test]
    fn rotate_yaw() {
        let v = Vector::new(1.0, 1.0, 1.0);
        let axis = Vector::new(0.0, 1.0, 0.0);

        compare_delta(
            v.rotate(&axis, PI / 2.0),
            Vector::new(1.0, 1.0, -1.0),
            1e-15,
        );
        compare_delta(
            v.rotate(&axis, -PI / 2.0),
            Vector::new(-1.0, 1.0, 1.0),
            1e-15,
        );
    }

    #[test]
    fn rotate_roll() {
        let v = Vector::new(1.0, 1.0, 1.0);
        let axis = Vector::new(0.0, 0.0, 1.0);

        compare_delta(
            v.rotate(&axis, PI / 2.0),
            Vector::new(-1.0, 1.0, 1.0),
            1e-15,
        );
        compare_delta(
            v.rotate(&axis, -PI / 2.0),
            Vector::new(1.0, -1.0, 1.0),
            1e-15,
        );
    }

    fn compare_delta(v1: Vector, v2: Vector, delta: f64) {
        assert_delta!(v1.x, v2.x, delta);
        assert_delta!(v1.y, v2.y, delta);
        assert_delta!(v1.z, v2.z, delta);
    }

    #[test]
    fn reflect() {
        let v = Vector::new(-1.0, -1.0, -1.0);
        let normal = Vector::new(1.0, 0.0, 0.0);

        assert_eq!(v.reflect(&normal), Vector::new(1.0, -1.0, -1.0));
    }
}
