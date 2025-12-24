use radar_toolbox::systems::radar_simple;
       
pub fn solve(){
    println!("---- Solving problem 14 for chapter 2 ----");

    let radar = radar_simple::SimpleRadarBuilder::new("problem 14")
        .transmit_power(100E3)
        .center_frequency(9.4E9)
        .atmospheric_loss(0.04)
        .transmit_gain(10.0_f64.powf(32.0/10.0))
        .effective_aperture(1.2)
        .build().unwrap();

    let jammer = radar_simple::SimpleRadarBuilder::new("jammer")
        .transmit_power(100.0)
        .center_frequency(9.4E9)
        .transmit_gain(10.0_f64.powf(15.0/10.0))
        .atmospheric_loss(0.04)
        .build().unwrap();
    

    let pr = radar.jamming_power(&jammer, 100E3);

    println!("The jamming power from the jammer is {:.2e} Watts", pr);
    println!("The jamming power from the jammer is {:.2e} dBm", 10.0*pr.log10() + 30.0);


}
