//use bigdecimal::{BigDecimal, Zero};
//use std::str::FromStr;
fn main() {
    let mut flip = true;
    let mut pi = 0f64;
    for x in (2..2642246).step_by(2) {
        let f = x * (x + 1) * (x + 2) as u64;
        let p = 4f64 / f as f64;
        pi = if flip { pi + &p } else { pi - &p };
        print!("4/({}*{}*{}) => ", x, x + 1, x + 2);
        println!("{}", p);
        flip = !flip;
    }
    println!("PI={:.}", 3f64 + pi);
}
