use radar_toolbox::signals::propagation::{multipath_dr, multipath_gr};
pub fn solve(){
    println!("---- Solving problem 13 for chapter 4 ----");

    let dr = multipath_dr(2.0, 4.0, 100.0);
    let gr = multipath_gr(2.0, 4.0, 100.0);

    println!("extra range and grazing angle: ({:.2}, {:.2})", dr, gr.to_degrees());

}
