fn main() {
    let mut _flip = true;
    let mut total = 0.;
    for x in (1..1003).step_by(2) {
        let n = x.to_string().chars().count() as u32;
        let e = if n == 1 { 1f64 } else { (10u32.pow(n - 1)) as f64 };
        if _flip {
            let p = (4f64 * e) / (x as f64);
            print!("({}/{}) => ", (4f64 * &e), x);
            println!("+{:.}", p);
            total += p;
        } else {
            let p = (4f64 * e) / (x as f64);
            print!("({}/{}) => ", (4f64 * &e), x);
            println!("-{:.}", p);
            let inv_p = -p;
            total += inv_p;
        }
        _flip = !_flip;
    }
    println!("PI = {:.}", total);
    //println!("{:.}", 0.0000004004004004004004f64);
}
