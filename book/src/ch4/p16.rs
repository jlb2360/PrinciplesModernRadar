use radar_toolbox::signals::propagation::tracker_angle_error;

pub fn solve(){
    println!("---- Solving problem 16 for chapter 4 ----");

    let th_err = tracker_angle_error(0.8, 2.0_f64.to_radians(), 0.7);


    println!("Estimated Angle Error: {:.2} degrees", th_err.to_degrees());
}
