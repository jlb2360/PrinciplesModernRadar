use std::f64::consts::PI;



pub struct Sphere{
    pub a: f64,
    pub rcs: f64
}


impl Sphere {
    pub fn from_radius(a: f64)->Self{
        Sphere { a: a, rcs: PI*a*a } 
    }
}


pub struct CornerReflector{
    pub a: f64,
    pub rcs: f64
}


impl CornerReflector {
    pub fn from_length(a: f64, lam: f64)->Self{
        CornerReflector { a: a, rcs: 12.0*PI*a.powi(4)/(lam*lam) }
    }
    
    pub fn from_rcs(rcs: f64, lam: f64)->Self{
        CornerReflector { a: (rcs * lam * lam / (12.0 * PI)).powf(0.25), rcs: rcs }
    }

}
