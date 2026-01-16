use crate::{signals::traits::RadarWaveform, C};


pub struct PulsedRadar<W: RadarWaveform>{
    pub waveform: W,
    n_p: i64,
    pri: f64,
}

impl<W: RadarWaveform> PulsedRadar<W> {
    pub fn new(waveform: W, n_p: i64, pri: f64)->Self{
        PulsedRadar { waveform, n_p, pri }
    }

    pub fn doppler_resolution(&self)->f64{
        1.0/self.cpi()
    }

    pub fn cpi(&self)->f64{
        self.n_p as f64 * self.pri
    }

    pub fn range_ambiguity(&self)->f64{
        self.pri * C/ 2.0
    }

    pub fn doppler_ambiguity(&self)->f64{
        1.0/(2.0*self.pri)
    }
}

