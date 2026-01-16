
pub trait RadarWaveform {
    fn pulse_width(&self)->f64;
    fn bandwidth(&self)->f64;
    fn range_resolution(&self)->f64;

    // functions with defaults
    fn rayleigh_bandwidth(&self)->f64 {
        let pw = self.pulse_width();
        1.0/pw
    }

    fn bandwidth_3db(&self)->f64 {
        let pw = self.pulse_width();
        0.89/pw
    }
 
}


