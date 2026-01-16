use crate::{signals::traits::RadarWaveform, C};

pub struct CW{
    pub f_c: f64,
    pub t_p: f64
}

impl CW {
   pub fn new(f_c: f64, t_p: f64)->Self{
        CW { f_c, t_p }
    }
}

impl RadarWaveform for CW {

    fn pulse_width(&self)->f64 {
        self.t_p
    }

    fn bandwidth(&self)->f64 {
        1.0/self.t_p
    }

    fn range_resolution(&self)->f64 {
        C * self.t_p / 2.0
    }
    
}
