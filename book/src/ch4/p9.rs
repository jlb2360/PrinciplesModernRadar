use radar_toolbox::signals::propagation::diffraction_knife;
pub fn solve(){
    println!("---- Solving problem 9 for chapter 4 ----");

    let f = 1.0*10.0_f64.powf(diffraction_knife(15.0_f64.to_radians())/10.0);


    println!("Magnitude of reflection {:.2}", f);
}
