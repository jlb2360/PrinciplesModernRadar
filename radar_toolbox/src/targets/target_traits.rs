
pub trait Pfa{
    fn pfa(&self, threshold: f64)->f64;

    fn threshold(&self, pfa: f64)->f64{
        println!("Function is not defined");
        pfa
    }

    fn n_dwells(&self, pfa_desired: f64, pfa_current: f64) -> i32{
        let mut n = 1;
        let mut new_pfa = pfa_current;
        while new_pfa > pfa_desired {
            new_pfa *= pfa_current;
            n += 1;
        }

        return n;
    }
}

pub trait Pd{
    fn snr(&self, pd: f64, pfa: f64)->f64{
        println!("Function is not defined");
        0.0
    }
}

