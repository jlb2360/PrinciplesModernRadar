use radar_toolbox::systems::radar_simple;
       
pub fn solve(){
    println!("---- Solving problem 3 for chapter 4 ----");

    let radar = radar_simple::SimpleRadarBuilder::new("problem 3")
        .center_frequency(1E9)
        .build().unwrap();
    

    let dist = radar_simple::SimpleRadar::range(10E-6);
    let ph = radar.phase_change(10E-6);



    println!("The distance traveled is {:.2e} m", dist);
    println!("The phase change over distance is {:.2} radians", ph);

}
