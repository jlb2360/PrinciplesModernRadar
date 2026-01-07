       
pub fn solve(){
    println!("---- Solving problem 1 for chapter 3 ----");

    let az_width = 2.0;
    let el_width = 3.0;

    let az_scan = 90.0;
    let el_scan = 6.0;

    let tot_scans = az_scan / az_width * el_scan / el_width;

    println!("The total number of scans necessary are {}", tot_scans);
}
