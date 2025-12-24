use radar_toolbox::systems::radar_simple::{self};
       
pub fn solve(){
    println!("---- Solving problem 13 for chapter 2 ----");

    let radar = radar_simple::SimpleRadarBuilder::new("problem 1")
        .transmit_power(100E3)
        .center_frequency(9.4E9)
        .transmit_gain(10.0_f64.powf(32.0/10.0))
        .receiver_gain(10.0_f64.powf(32.0/10.0))
        .build().unwrap();
    

    let pr = radar.scr(10_f64.powf(0.0/10.0), 50E3, 10_f64.powf(-70.0/10.0), 900000000.0, radar_simple::ClutterType::VOLUME);


    println!("The SCR for the radar is {:.2} dB", 10.0*pr.log10());
}
