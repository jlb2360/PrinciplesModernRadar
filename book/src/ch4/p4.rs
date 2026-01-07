        
pub fn solve(){
    println!("---- Solving problem 4 for chapter 4 ----");

    let rain = 1.0 * 2.0 / 2.0;
    let clear_air = 0.3 * 1.0 / 2.0;
    let fog = 0.7 * 2.0 / 2.0;

    let prop_loss = -1.0 * (rain + clear_air + fog);


    println!("The propagation loss {:.2}", prop_loss);
}
