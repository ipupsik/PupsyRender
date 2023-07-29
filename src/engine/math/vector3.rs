
// Rust program for Operator Overloading
use std::ops::{Add, Sub, Mul, Div};

#[derive(Copy, Clone)]
pub struct Vector3 {
    pub vec : [f64; 3]
}

impl Vector3 {
    pub fn new(x : f64, y : f64, z : f64) -> Self {
        Vector3{vec: [x, y, z]}
    }

    pub fn x(&self) -> f64 {
        self.vec[0]
    }

    pub fn y(&self) -> f64 {
        self.vec[1]
    }

    pub fn z(&self) -> f64 {
        self.vec[2]
    }

    pub fn length(&self) -> f64 {
       (self.x() * self.x() + self.y() * self.y() + self.z() * self.z()).sqrt()
    }

    pub fn dot(&self, _rightside: Vector3) -> f64 {
        self.x() * _rightside.x() + self.y() * _rightside.y() + self.z() * _rightside.z() 
    }

    pub fn cross(&self, _rightside: Vector3) -> Vector3 {
        Vector3{vec: [self.y() * _rightside.z() - self.z() * _rightside.y(),
            self.z() * _rightside.x() - self.x() * _rightside.z(),
            self.x() * _rightside.y() - self.y() * _rightside.x()]}
    }

    pub fn normalize(&self) -> Vector3 {
        let length : f64 = self.length();
        Vector3{vec : [self.x() / length, self.y() / length, self.z() / length]}
    }
}

// Rust operator 
impl Mul<f64> for Vector3 {
    type Output = Vector3;

    fn mul(self, _rightside: f64) -> Vector3 {
        Vector3{ vec: [self.x() * _rightside, self.y() * _rightside, self.z() * _rightside] }
    }
}

// Rust operator 
impl Div<f64> for Vector3 {
    type Output = Vector3;

    fn div(self, _rightside: f64) -> Vector3 {
        Vector3{ vec: [self.x() / _rightside, self.y() / _rightside, self.z() / _rightside] }
    }
}

// Rust operator 
impl Mul<Vector3> for f64 {
    type Output = Vector3;

    fn mul(self, _rightside: Vector3) -> Vector3 {
        Vector3{ vec: [self * _rightside.x(), self * _rightside.y(), self * _rightside.x()] }
    }
}

// Rust operator 
impl Add<Vector3> for Vector3 {
    type Output = Vector3;

    fn add(self, _rightside: Vector3) -> Vector3 {
        Vector3{ vec: [self.x() + _rightside.x(), self.y() + _rightside.y(), self.z() + _rightside.z()]}
    }
}

// Rust operator 
impl Sub<Vector3> for Vector3 {
    type Output = Vector3;

    fn sub(self, _rightside: Vector3) -> Vector3 {
        Vector3{ vec: [self.x() - _rightside.x(), self.y() - _rightside.y(), self.z() - _rightside.z()]}
    }
}