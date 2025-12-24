use radar_toolbox::systems::radar_simple;
       
pub fn solve(){
    println!("---- Solving problem 17 for chapter 2 ----");

    let radar = radar_simple::SimpleRadarBuilder::new("problem 15")
        .transmit_power(100E3)
        .center_frequency(9.4E9)
        .atmospheric_loss(0.04)
        .transmit_gain(10.0_f64.powf(32.0/10.0))
        .effective_aperture(1.2)
        .noise_figure(10.0_f64.powf(2.7/10.0))
        .receiver_bandwidth(1E6)
        .build().unwrap();

    let jammer = radar_simple::SimpleRadarBuilder::new("jammer")
        .transmit_power(100.0)
        .center_frequency(9.4E9)
        .transmit_gain(10.0_f64.powf((15.0-30.0)/10.0))
        .atmospheric_loss(0.04)
        .noise_figure(10.0_f64.powf(2.7/10.0))
        .receiver_bandwidth(1E6)
        .build().unwrap();
    

    let pr = radar.sjr(&jammer, 100E3, 10_f64.powf(0.0/10.0), 50E3); 


    println!("The SJR for the system is {:2} dB", 10.0*pr.log10());


}
