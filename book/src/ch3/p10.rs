pub fn solve(){
    println!("---- Solving problem 10 for chapter 3 ----");

    let n_scans_az = (75.0_f64/2.0_f64).ceil();
    let n_scans = n_scans_az * 4.0/2.0;
    let mut scan_time_ms = n_scans * 2.4 * 5.0;

    let track_time_ms = scan_time_ms * 1E-3 * 10.0 * 6.0 * 8.0;

    scan_time_ms = track_time_ms + scan_time_ms;

    println!("The total scan time is {:.2} ms", scan_time_ms);

    
}
