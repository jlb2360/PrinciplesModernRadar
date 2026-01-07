pub fn solve(){
    println!("---- Solving problem 12 for chapter 3 ----");

    let d_acq = 20E3 * 2.0_f64.to_radians().tan();
    let d_track = 20E3 * 6.0_f64.to_radians().tan();

    let n_track_samps = 50.0;

    let v_mort = 200.0;

    let track_time = n_track_samps / ((d_track - d_acq)/v_mort);

    println!("The time needed to get necessarry track samples {:.2} Hz", track_time);
}
