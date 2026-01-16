use std::f64::consts::PI;

pub fn beam_width(alpha: f64, lam: f64, length: f64) -> f64{
    alpha * lam / length
}

pub fn max_directivity(eta: f64, area: f64, lam: f64) -> f64{
    eta * 4.0 * PI * area / (lam * lam)
}
