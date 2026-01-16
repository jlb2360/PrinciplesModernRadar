use std::f64::consts::PI;

use crate::core::types::Complex;
pub trait PhasedArray{
    fn antenna_factor(&self, th_rad: f64, wavelength: f64)->Complex;
    
    fn th3(&self, wavelength: f64)->f64;
}

pub struct LinearArray{
    spacing: f64,
    n_elements: usize,
    th_steered: f64
}

impl LinearArray{
    pub fn new(spacing: f64, n_elements: usize, th_steered: f64)->Self{
        LinearArray { spacing, n_elements, th_steered}
    }

    pub fn no_grating(th_steered: f64, wavelength: f64) -> f64{
        wavelength / (1.0 + th_steered.sin().abs()) 
    }

    pub fn elements_needed_for_th3(th3_rad: f64, spacing: f64, wavelength: f64, th_steered: f64) -> usize{
        (0.886 * wavelength / (th3_rad * spacing * th_steered.cos())).ceil() as usize 
    }
}

impl PhasedArray for LinearArray{

    fn antenna_factor(&self, th_rad: f64, wavelength: f64)->Complex{
        let af: Complex = (1..=self.n_elements)
            .map(|n| 
                {
                    Complex::new(
                        0.0, 
                        2.0*PI/wavelength * 
                        n as f64 * 
                        self.spacing * 
                        (th_rad.sin() - self.th_steered.sin())).exp()
            }
        ).sum();

        return af / (self.n_elements as f64);
    }

    fn th3(&self, wavelength: f64)->f64 {
        wavelength/(self.n_elements as f64 * self.spacing)
    }

}
