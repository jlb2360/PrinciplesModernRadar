pub fn solve(){
    println!("---- Solving problem 11 for chapter 2 ----");

    let tau_1 = 1.75;
    let sig1 = 1.0;
    let sig2 = 0.1;


    let tau_2 = tau_1 * sig1/sig2;
    println!("The dwell time needed is {:.2}", tau_2);
}
