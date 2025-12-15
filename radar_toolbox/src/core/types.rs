use std::ops::{Add, Sub, Mul, Div, Neg};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Complex{
    pub re: f64,
    pub im: f64
}


impl Complex{
    pub fn new(re: f64, im: f64) -> Self { Self { re: re, im: im }}
    
    pub fn zero() -> Self { Self { re: 0.0, im: 0.0 }}

    pub fn exp(theta: f64) -> Self {
        Self { re: theta.cos(), im: theta.sin() }
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

impl Mul<Complex> for f64 {
    type Output = Complex;
    fn mul(self, complex: Complex) -> Complex {
        Complex { re: self * complex.re, im: self * complex.im }
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
