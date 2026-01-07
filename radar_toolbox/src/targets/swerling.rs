use crate::targets::target_traits::Pd;

pub struct Sw1{
    rcs: f64
}

impl Sw1{
    pub fn new(rcs: f64)->Self{
        Sw1 { rcs: rcs }
    }
}

// * this assumes a thermal noise background * //
impl Pd for Sw1 {
    fn snr(&self, pd: f64, pfa: f64)->f64 {
        pfa.ln() / pd.ln() - 1.0
    } 
}
