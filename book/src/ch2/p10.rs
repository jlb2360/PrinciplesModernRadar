pub fn solve(){
    println!("---- Solving problem 10 for chapter 2 ----");

    let snr = 10.0_f64.powf(13.0/10.0);
    let np = 20.0;

    let snr_new = 10.0*(snr*np).log10();

    println!("The integrated snr is {:.2}", snr_new);
}
