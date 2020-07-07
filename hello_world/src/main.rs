fn main() {
    let mut flip = true;
    let mut total = 0.;
    for i in (1..1003).step_by(2) {
        let x = i as f64;
        let n = i.to_string().chars().count() as u32;
        let e = 10u32.pow(n - 1) as f64;
        let p = (4f64 * e) / x;
        print!("({}/{}) => ", (4f64 * &e), x);
        if flip {
            println!("+{:.}", p);
            total += p;
        } else {
            println!("-{:.}", p);
            total += -p;
        }
        flip = !flip;
    }
    println!("PI = {:.}", total);
    //println!("{:.}", 0.0000004004004004004004f64);
}
