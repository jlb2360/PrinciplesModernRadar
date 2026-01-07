use crate::targets::target_traits;

pub struct ThermalNoise{
    v: f64,
    sig_n: f64
}

impl ThermalNoise{
    pub fn new(v: f64, sig_n: f64) -> Self{
        ThermalNoise { v: v, sig_n: sig_n }
    }
}

impl target_traits::Pfa for ThermalNoise {
    fn pfa(&self, threshold: f64)->f64 {
        (-threshold * threshold / (2.0 * self.sig_n * self.sig_n)).exp() 
    }    

    fn threshold(&self, pfa: f64)->f64 {
        (2.0*self.sig_n * self.sig_n * (1.0/pfa).ln()).sqrt()
    }
}
