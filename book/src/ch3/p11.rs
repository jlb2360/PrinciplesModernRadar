pub fn solve(){
    println!("---- Solving problem 11 for chapter 3 ----");

    let d_fence = 20E3 * 4.0_f64.to_radians().tan();

    let v_mort = 200.0;

    let t_fence = d_fence / v_mort;

    let scan_time = t_fence/4.0;

    println!("The scan time needed to have 4 attempts is {:.2} s", scan_time);
}
