fn main() {
    let mut _flip = true;
    let mut total = 0.;
    for x in (1..90).step_by(2) {
        let p = 4.0 / x as f64;
        if _flip {
            println!("{}", p);
            total = total + p;
        } else {
            println!("{}", p);
            let inv_p = -p;
            total += inv_p;
        }
        _flip = !_flip;
    }
    println!("PI = {:.}", total);
}
