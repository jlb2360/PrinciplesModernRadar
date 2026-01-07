         
pub fn solve(){
    println!("---- Solving problem 3 for chapter 3 ----");

    let az_width = 2.0;
    let el_width = 3.0;

    let az_scan = 90.0;
    let el_scan = 6.0;

    let scan_rate = 180.0;

    let dwell_time = az_width/scan_rate;
    let tot_scans = az_scan / az_width * el_scan / el_width;

    let scan_time = dwell_time * tot_scans;

    println!("The total scan time necessary is {} s", scan_time);
}
