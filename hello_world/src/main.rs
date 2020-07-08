fn main() {
    let mut flip = true;
    let mut total = 0.;
    for i in (1..1003).step_by(2) {
        let x = i as f64;
        let n = i.to_string().chars().count() as u32;
        let e = 10u32.pow(n - 1) as f64;
        let p = (4f64 * e) / x;
        let str_p = p.to_string().replace('.', "");
        let arr = vec!['0'; (n - 1) as usize];
        let s1: String = arr.iter().collect();
        let mut s2 = s1 + &str_p;
        s2.insert(1, '.');
        let p2: f64 = s2.parse().unwrap();
        print!("({}/{}) => ", (4f64 * &e), x);
        if flip {
            println!("+{:.}", p2);
            total += p2;
        } else {
            println!("-{:.}", p2);
            total += -p2;
        }
        flip = !flip;
    }
    println!("PI = {:.}", total);
    //println!("{:.}", 0.0000004004004004004004f64);
}
