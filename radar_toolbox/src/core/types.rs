use std::{iter::Sum, ops::{Add, Div, Mul, Neg, Sub}};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Complex{
    pub re: f64,
    pub im: f64
}


impl Complex{
    pub fn new(re: f64, im: f64) -> Self { Self { re: re, im: im }}
    
    pub fn zero() -> Self { Self { re: 0.0, im: 0.0 }}

    pub fn from_polar(theta: f64) -> Self {
        Self { re: theta.cos(), im: theta.sin() }
    }

    pub fn exp(&self)->Self{
        let exp_re = self.re.exp();
        Self {
            re: exp_re * self.im.cos(),
            im: exp_re * self.im.sin(),
        }
    }

    pub fn abs(&self) -> f64 {
        self.re.hypot(self.im)
    }

    pub fn abs_sq(&self) -> f64 {
        self.re * self.re + self.im * self.im
    }


    pub fn phase(&self) -> f64 {self.im.atan2(self.re)}

    pub fn conj(&self) -> Self { Self { re: self.re, im: -self.im }}



}


impl Neg for Complex {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self { re: -self.re, im: -self.im }
    }
}

impl Add for Complex{
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {re: self.re + other.re, im: self.im + other.im}
    }
}

impl Sub for Complex{
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {re: self.re - other.re, im: self.im - other.im}
    }
}

impl Mul for Complex{
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Self {
            re: self.re * other.re - self.im * other.im,
            im: self.re * other.im + self.im * other.re
        }
    }

}

impl Mul<f64> for Complex{
    type Output = Self;
    fn mul(self, other: f64) -> Self{
        Self{re: self.re*other, im: self.im*other}
    }
}

impl Add<f64> for Complex{
    type Output = Self;
    fn add(self, rhs: f64) -> Self::Output {
        Self{re: self.re + rhs, im: self.im}
    }
}

impl Mul<Complex> for f64 {
    type Output = Complex;
    fn mul(self, complex: Complex) -> Complex {
        Complex { re: self * complex.re, im: self * complex.im }
    }
}

impl Add<Complex> for f64 {
    type Output = Complex;
    fn add(self, rhs: Complex) -> Self::Output {
        Complex { re: self + rhs.re, im: rhs.im }
    }
}


impl Div for Complex {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        let den = other.re * other.re + other.im * other.im;
        if den == 0.0 {
            panic!("Division by zero in Complex numbers");
        }
        Self {
            re: (self.re * other.re + self.im * other.im) / den,
            im: (self.im * other.re - self.re * other.im) / den,
        }
    }
}

impl Div<f64> for Complex {
    type Output = Self;
    fn div(self, scalar: f64) -> Self {
        if scalar == 0.0 {
            panic!("Division by zero");
        }
        Self { re: self.re / scalar, im: self.im / scalar }
    }
}

impl Sum for Complex {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Complex { re: 0.0, im: 0.0 }, |a, b| a + b)
    }
}

impl<'a> Sum<&'a Complex> for Complex {
    fn sum<I: Iterator<Item = &'a Complex>>(iter: I) -> Self {
        iter.fold(Complex { re: 0.0, im: 0.0 }, |a, b| Complex {
            re: a.re + b.re,
            im: a.im + b.im,
        })
    }
}



#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3{
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self{
        Vec3{x, y, z}
    }

    pub fn dot(self, other: Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn length(self) -> f64{
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    pub fn normalize(self) -> Vec3 {
        let len = self.length();
        Vec3 {
            x: self.x/len,
            y: self.y/len,
            z: self.z/len,
        }
    }
}


impl Add for Vec3 {
    type Output = Self;
    fn add(self, other: Self) -> Self{
        Vec3::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, other: Self) -> Self{
        Vec3::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}




// * Pipe trait which lets you chain functions
pub trait Pipe: Sized {
    fn pipe<F, R>(self, f: F) -> R
    where
        F: FnOnce(Self) -> R,
    {
        f(self)
    }
}

// Implement it for every type T
impl<T> Pipe for T {}


#[cfg(test)]
mod tests {
    use super::*; // Import everything from the parent module

    // Helper for floating point comparisons
    fn assert_approx_eq(a: f64, b: f64) {
        let epsilon = 1e-10;
        assert!((a - b).abs() < epsilon, "Expected {}, got {}", b, a);
    }

    #[test]
    fn test_complex_addition() {
        let a = Complex::new(1.0, 2.0);
        let b = Complex::new(3.0, 4.0);
        let result = a + b;
        assert_eq!(result, Complex::new(4.0, 6.0));
    }

    #[test]
    fn test_complex_subtraction() {
        let a = Complex::new(1.0, 5.0);
        let b = Complex::new(3.0, 2.0);
        let result = a - b;
        assert_eq!(result, Complex::new(-2.0, 3.0));
    }

    #[test]
    fn test_complex_multiplication() {
        let a = Complex::new(1.0, 1.0); // 1 + i
        let b = Complex::new(1.0, -1.0); // 1 - i
        let result = a * b; 
        // (1+i)(1-i) = 1 - i^2 = 2
        assert_approx_eq(result.re, 2.0);
        assert_approx_eq(result.im, 0.0);
    }

    #[test]
    fn test_scalar_multiplication() {
        let a = Complex::new(2.0, 3.0);
        let result = a * 2.0;
        assert_eq!(result, Complex::new(4.0, 6.0));

        // Test reverse order (f64 * Complex)
        let result2 = 3.0 * a;
        assert_eq!(result2, Complex::new(6.0, 9.0));
    }

    #[test]
    fn test_complex_division() {
        // (10 + 5i) / (2 + i) should be 5
        let a = Complex::new(10.0, 5.0);
        let b = Complex::new(2.0, 1.0);
        let result = a / b;
        assert_approx_eq(result.re, 5.0);
        assert_approx_eq(result.im, 0.0);

        // Test scalar division
        let result_scalar = a / 2.0;
        assert_eq!(result_scalar, Complex::new(5.0, 2.5));
    }

    #[test]
    fn test_conjugate() {
        let a = Complex::new(3.0, -4.0);
        let c = a.conj();
        assert_eq!(c, Complex::new(3.0, 4.0));
    }

    #[test]
    fn test_magnitude() {
        // 3-4-5 triangle
        let a = Complex::new(3.0, 4.0);
        assert_approx_eq(a.abs(), 5.0);
    }
}
