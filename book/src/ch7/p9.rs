use radar_toolbox::core::constants::C;
pub fn solve(){
    println!("---- Solving problem 9 for chapter 7 ----");

    let l1 = 3.0;
    let l2 = 10.0;

    let df1 = C/(2.0 * l1);
    let df2 = C/(2.0*l2);

    if df1 > df2 {
        println!("
            The frequency decorrelation is maximum 
            at a zero degree aspect angle with frequency change {:.2e}",
            df1
        );
    } else {
        println!("
            The frequency decorrelation is maximum 
            at a 90 degree aspect angle with frequency change {:.2e}",
            df2
        );
    }

}
