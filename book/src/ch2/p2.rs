use radar_toolbox::systems::radar_simple;
       
pub fn solve(){
    println!("---- Solving problem 2 for chapter 2 ----");

    let radar = radar_simple::SimpleRadarBuilder::new("problem 1")
        .center_frequency(9.4E9)
        .noise_figure(10.0_f64.powf(2.7/10.0))
        .receiver_bandwidth(1E6)
        .build().unwrap();

    let nr = radar.noise_power();


    println!("The noise power for the radar is {:.2e} Watts", nr);


}
