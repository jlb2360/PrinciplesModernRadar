pub fn solve(){
    println!("---- Solving problem 8 for chapter 5 ----");

    let n_t = 30.0;
    let t_0 = 200E-6;

    let mut pri = 1.0/1E3;

    let mut n_indep = 1.0 + (pri * (n_t-1.0))/t_0;

    if n_indep > n_t {
        n_indep = n_t;
    }

    println!("The number of independent samples for prf = 1 KHz {}", n_indep);

    pri = 1.0/5E3;
    n_indep = 1.0 + (pri * (n_t-1.0))/t_0;

    if n_indep > n_t {
        n_indep = n_t;
    }


    println!("The number of independent samples for prf = 5 KHz {}", n_indep);
    
    pri = 1.0/40E3;
    n_indep = 1.0 + (pri * (n_t-1.0))/t_0;

    if n_indep > n_t {
        n_indep = n_t;
    }
    println!("The number of independent samples for prf = 40 KHz {}", n_indep);


}
