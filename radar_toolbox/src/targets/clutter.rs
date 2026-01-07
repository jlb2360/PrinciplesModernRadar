use std::f64::consts::PI;


pub fn pulse_limited_gr(th: f64, r: f64, beta: f64)->f64{
    (PI * r * (th/2.0).tan() / (2.0 * beta)).atan()  
}

// * assume small angle approximation
pub fn clutter_volume(th: f64, ph: f64, r: f64, beta: f64)->f64{
    PI * (r*th/2.0) * (r*ph/2.0) * beta
}

pub fn critical_gr(sig_h: f64, lam: f64)->f64{
    (lam / (8.0 * sig_h)).asin()    
}
