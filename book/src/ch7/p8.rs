use radar_toolbox::core::constants::C;
pub fn solve(){
    println!("---- Solving problem 8 for chapter 7 ----");

    let l = 3.0;

    let df = C/(2.0 * l);
    let f = 5E9;

    let rat = 9.0 * df / f;


    println!("The decorrelation frequency is {:.2e} Hz", df);
    println!("Ratio between change in frequency and frequency {:.2}", rat);

}
