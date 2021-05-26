pub fn add(s: &mut Vec<f64>) {
    if s.len() >= 2 {
        let f1 = s.pop().unwrap();
        let f2 = s.pop().unwrap();
        println!("\top: {} + {}", f1, f2);
        let res = f1 + f2;
        s.push(res)
    }
}

pub fn sub(s: &mut Vec<f64>) {
    if s.len() >= 2 {
        let f1 = s.pop().unwrap();
        let f2 = s.pop().unwrap();
        println!("\top: {} - {}", f1, f2);
        let res = f1 - f2;
        s.push(res)
    }
}

pub fn quit() {
    std::process::exit(0)
}
