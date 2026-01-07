use std::f64::consts::PI;

use crate::Complex;

// this should be shifted to a trait or struct methods? //
pub fn diffraction_knife(theta: f64) -> f64 {
    1.0/(2.0 * (2.0 * PI).sqrt()) * (
        1.0/(1.0/2.0 * (theta + PI)).cos() + 
        1.0/(1.0/2.0 * (theta + PI)).sin()
    )
}

pub fn multipath_dr(ht: f64, hr: f64, r: f64)->f64{
    2.0 * ht * hr / r
}

pub fn multipath_gr(ht: f64, hr: f64, r: f64)->f64{
    ((ht + hr) / r).atan()
}

pub fn multipath_attenuation_factor(k: f64, dr: f64, gamma: Complex)->Complex{
    gamma * (k * dr).cos() + 1.0
}

pub fn tracker_angle_error(rho_s: f64, th_3: f64, g_ratio: f64)->f64{
    rho_s * th_3 / (8.0 * g_ratio).sqrt()
}
