use radar_toolbox::systems::radar_simple;
       
pub fn solve(){
    println!("---- Solving problem 1 for chapter 2 ----");

    let radar = radar_simple::SimpleRadarBuilder::new("problem 1")
        .transmit_power(100E3)
        .center_frequency(9.4E9)
        .transmit_gain(10.0_f64.powf(32.0/10.0))
        .receiver_gain(10.0_f64.powf(32.0/10.0))
        .build().unwrap();
    

    let pr = radar.radar_range_equation(10_f64.powf(0.0/10.0), 50E3);


    println!("The received power from the target is {:.2e} Watts", pr);


}
