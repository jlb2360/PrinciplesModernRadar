use radar_toolbox::core::constants::C;
pub fn solve(){
    println!("---- Solving problem 10 for chapter 7 ----");

    let r_amb = 60.0 * 1852.0;
    let pri = 2.0 * r_amb/C;

    let omega = 12.5 * 360.0 / 60.0; // deg per s

    let n_pulses = (1.4 / (omega * pri)).floor();
    

    println!("The number of pulses while in main beam is {}", n_pulses);


}
