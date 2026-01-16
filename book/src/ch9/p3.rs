use std::f64::consts::PI;

use radar_toolbox::core::constants::C;
use radar_toolbox::antennas::simple::{max_directivity, beam_width};
pub fn solve(){
    println!("---- Solving problem 3 for chapter 9 ----");
    
    let mut lam = C/10E9;
    let rad = 0.5;

    let area = PI * rad * rad;

    let mut dir = max_directivity(1.0, area, lam);
    let mut bw = beam_width(1.0, lam, 2.0*rad);

    println!("Max directivity for 10 GHz is {:.2} dB", 10.0*dir.log10());
    println!("Beam width for 10 GHz is {:.2} dB", bw.to_degrees());
   
    lam = C/35E9;
    dir = max_directivity(1.0, area, lam);
    bw = beam_width(1.0, lam, 2.0*rad);

    println!("Max directivity for 35 GHz is {:.2} dB", 10.0*dir.log10());
    println!("Beam width for 35 GHz is {:.2} dB", bw.to_degrees());

}
