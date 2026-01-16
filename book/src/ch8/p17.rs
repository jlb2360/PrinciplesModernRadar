use radar_toolbox::signals::pulses::CW;
use radar_toolbox::systems::pulse_radar::PulsedRadar;
pub fn solve(){
    println!("---- Solving problem 17 for chapter 8 ----");

    let wave = CW::new(35E9, 10E-6);

    let radar = PulsedRadar::new(wave, 30, 1.0/5E3);

    let r_amb = radar.range_ambiguity();

    println!("The Range ambiguity is {:.2} m", r_amb);

    let tar_pos = 50E3;
    let m = (tar_pos/r_amb).floor();
    let r_meas = tar_pos - m*r_amb;

    println!("The number of pulses until the measurement is {}", m);
    println!("The measured range is {:.2} m", r_meas);

}
