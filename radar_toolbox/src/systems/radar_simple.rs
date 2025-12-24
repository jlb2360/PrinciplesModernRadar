use crate::core::types::Pipe;
use crate::core::constants::*;
use crate::systems::physics;


pub enum ClutterType {
    SURFACE,
    VOLUME,
}

#[derive(Debug, Clone)]
pub struct SimpleRadar{
    pub name: String,
    pt: f64,
    gt: f64,
    gr: f64,
    fc: f64,
    rec_f: f64,
    rec_b: f64,
    l_tx: f64,
    l_rx: f64,
    l_atm_dB_km: f64,
    ae: f64
}

impl SimpleRadar{

    pub fn snr(&self, rcs: f64, range: f64) ->f64{
        self.radar_range_equation(rcs, range)/self.noise_power()
    }

    pub fn sjr(&self, other: &SimpleRadar, range_j: f64, rcs: f64, range_t: f64)->f64{
        self.radar_range_equation(rcs, range_t)/(self.noise_power() + self.jamming_power(other, range_j)) 
    }

    pub fn scr(&self, rcs: f64, range: f64, rcs_0: f64, space: f64, ctype: ClutterType)->f64{
        self.radar_range_equation(rcs, range)/(self.clutter_return(rcs_0, space, range, ctype))
    }

    pub fn noise_power(&self)->f64{
       BOLTZMANN * STD_TEMP * self.rec_f * self.rec_b 
    }
    
    pub fn radar_range_equation(&self, rcs: f64, range: f64)->f64{
        if self.ae != 0.0{
            self.pt
                .pipe(|pc| physics::chain_loss(pc, self.l_tx))
                .pipe(|pc| physics::transmission_f(pc, self.gt))
                .pipe(|pc| physics::propogation(pc, range))
                .pipe(|pc| physics::atm_attenuation(pc, range, self.l_atm_dB_km))
                .pipe(|pc| physics::reflection(pc, rcs))
                .pipe(|pc| physics::propogation(pc, range))
                .pipe(|pc| physics::atm_attenuation(pc, range, self.l_atm_dB_km))
                .pipe(|pc| physics::reception_ae(pc, self.ae))
                .pipe(|pc| physics::chain_loss(pc, self.l_rx))
        } else {
            self.pt
                .pipe(|pc| physics::chain_loss(pc, self.l_tx))
                .pipe(|pc| physics::transmission_f(pc, self.gt))
                .pipe(|pc| physics::propogation(pc, range))
                .pipe(|pc| physics::atm_attenuation(pc, range, self.l_atm_dB_km))
                .pipe(|pc| physics::reflection(pc, rcs))
                .pipe(|pc| physics::propogation(pc, range))
                .pipe(|pc| physics::atm_attenuation(pc, range, self.l_atm_dB_km))
                .pipe(|pc| physics::reception(pc, C/self.fc, self.gr))
                .pipe(|pc| physics::chain_loss(pc, self.l_rx))
        }
    }

    pub fn jamming_power(&self, other: &SimpleRadar, range: f64)->f64{
        if self.ae != 0.0{
            other.pt
                .pipe(|pc| physics::chain_loss(pc, other.l_tx))
                .pipe(|pc| physics::transmission_f(pc, other.gt))
                .pipe(|pc| physics::propogation(pc, range))
                .pipe(|pc| physics::atm_attenuation(pc, range, other.l_atm_dB_km))
                .pipe(|pc| physics::reception_ae(pc, self.ae))
                .pipe(|pc| physics::chain_loss(pc, self.l_rx))
        } else {
            other.pt
                .pipe(|pc| physics::chain_loss(pc, other.l_tx))
                .pipe(|pc| physics::transmission_f(pc, other.gt))
                .pipe(|pc| physics::propogation(pc, range))
                .pipe(|pc| physics::atm_attenuation(pc, range, other.l_atm_dB_km))
                .pipe(|pc| physics::reception(pc, C/self.fc, self.gr))
                .pipe(|pc| physics::chain_loss(pc, self.l_rx))
        }
    }


    pub fn clutter_return(&self, rcs_0: f64, space: f64, range: f64, ctype: ClutterType)->f64{
        match ctype {
            ClutterType::SURFACE => self.surface_clutter_return(rcs_0, range, space),
            ClutterType::VOLUME => self.volume_clutter_return(rcs_0, range, space)
        }
    }

    pub fn volume_clutter_return(&self, rcs_0: f64, range: f64, volume: f64)->f64{
         self.pt
            .pipe(|pc| physics::chain_loss(pc, self.l_tx))
            .pipe(|pc| physics::transmission_f(pc, self.gt))
            .pipe(|pc| physics::propogation(pc, range))
            .pipe(|pc| physics::atm_attenuation(pc, range, self.l_atm_dB_km))
            .pipe(|pc| physics::reflection(pc, rcs_0 * volume))
            .pipe(|pc| physics::propogation(pc, range))
            .pipe(|pc| physics::atm_attenuation(pc, range, self.l_atm_dB_km))
            .pipe(|pc| physics::reception(pc, C/self.fc, self.gr))
            .pipe(|pc| physics::chain_loss(pc, self.l_rx))
    }

    pub fn surface_clutter_return(&self, rcs_0: f64, range: f64, area: f64)->f64{
        self.pt
            .pipe(|pc| physics::chain_loss(pc, self.l_tx))
            .pipe(|pc| physics::transmission_f(pc, self.gt))
            .pipe(|pc| physics::propogation(pc, range))
            .pipe(|pc| physics::atm_attenuation(pc, range, self.l_atm_dB_km))
            .pipe(|pc| physics::reflection(pc, rcs_0 * area))
            .pipe(|pc| physics::propogation(pc, range))
            .pipe(|pc| physics::atm_attenuation(pc, range, self.l_atm_dB_km))
            .pipe(|pc| physics::reception(pc, C/self.fc, self.gr))
            .pipe(|pc| physics::chain_loss(pc, self.l_rx))

    }

}


pub struct SimpleRadarBuilder{
    name: String,
    pt: Option<f64>,
    gt: Option<f64>,
    gr: Option<f64>,
    fc: Option<f64>,
    rec_f: Option<f64>,
    rec_b: Option<f64>,
    l_tx: Option<f64>,
    l_rx: Option<f64>,
    l_atm_dB_km: Option<f64>,
    ae: Option<f64>
}

impl SimpleRadarBuilder {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            pt: None,
            gt: None,
            gr: None,
            fc: None,
            rec_f: None,
            rec_b: None,
            l_tx: None,
            l_rx: None,
            l_atm_dB_km: None,
            ae: None,
        }
    }

    pub fn transmit_power(mut self, pt: f64)->Self{
        self.pt = Some(pt);
        self
    }

    pub fn transmit_gain(mut self, gt: f64)->Self{
        self.gt = Some(gt);
        self
    }

    pub fn receiver_gain(mut self, gr: f64)->Self{
        self.gr = Some(gr);
        self
    }

    pub fn center_frequency(mut self, fc: f64)->Self{
        self.fc = Some(fc);
        self
    }

    pub fn noise_figure(mut self, f: f64)->Self{
        self.rec_f = Some(f);
        self
    }

    pub fn receiver_bandwidth(mut self, b: f64)->Self{
        self.rec_b = Some(b);
        self
    }

    pub fn transmit_loss(mut self, l_tx: f64)->Self{
        self.l_tx = Some(l_tx);
        self
    }

    pub fn receiver_loss(mut self, l_rx: f64)->Self{
        self.l_rx = Some(l_rx);
        self
    }

    pub fn atmospheric_loss(mut self, l_atm: f64)->Self{
        self.l_atm_dB_km = Some(l_atm);
        self
    }
    
    pub fn effective_aperture(mut self, ae: f64)->Self{
        self.ae = Some(ae);
        self
    }
    pub fn build(self)->Result<SimpleRadar, String>{
        Ok(SimpleRadar { 
            name: self.name, 
            pt: self.pt.unwrap_or(5.0), // default to 5 watt system 
            gt: self.gt.unwrap_or(1.0), // default to no gain 
            gr: self.gr.unwrap_or(1.0), // default to no gain 
            fc: self.fc.ok_or("Frequency is required")?,
            rec_f: self.rec_f.unwrap_or(1.0), // default to linear noise figure of 1
            rec_b: self.rec_b.unwrap_or(1E6), // default to a 1 MHz receiver bandwidth
            l_tx: self.l_tx.unwrap_or(1.0), // default to no transmit loss
            l_rx: self.l_rx.unwrap_or(1.0), // default to no receive loss
            l_atm_dB_km: self.l_atm_dB_km.unwrap_or(0.0), // default to no atmospheric loss
            ae: self.ae.unwrap_or(0.0)
        })
    }
}
