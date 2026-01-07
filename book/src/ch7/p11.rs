use radar_toolbox::core::constants::C;
pub fn solve(){
    println!("---- Solving problem 11 for chapter 7 ----");

    let r_amb = 60.0 * 1852.0;
    let pri = 2.0 * r_amb/C;
    let v = 120.0 * 1.94384; // m per s
    let r = 50.0 * 1852.0;

    let lam = C/2.8E9;
    let th_cor = lam/(47.0 * 2.0) * 1E3;

    let omega = 12.5 * 360.0 / 60.0; // deg per s

    let n_pulses = (1.4 / (omega * pri)).floor();

    let dtheta = (v * n_pulses * pri / r).atan() * 1E3;
    

    println!("The change in aspect angle over a scan is {:.3} mrad", dtheta);
    println!("The decorrelation distance is {:.3} mrad", th_cor);

    if th_cor > dtheta{
        println!("Use scan to scan for decorrelation");
    } else {
        println!("Use pulse to pulse for decorrelation");
    }


}
