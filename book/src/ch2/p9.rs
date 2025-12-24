pub fn solve(){
    println!("---- Solving problem 9 for chapter 2 ----");

    let sig1 = 1.0;
    let mut sig2 = 0.5;
    let r: f64 = 50.0E3_f64.powf(4.0);

    let mut r2 = sig2/sig1 * r;

    println!("a \n\t Equivalent range is {:.2}", r2.powf(0.25));

    sig2 = 0.1;
    
    r2 = sig2/sig1 * r;

    println!("b \n\t Equivalent range is {:.2}", r2.powf(0.25));

}
