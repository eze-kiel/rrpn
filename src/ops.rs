pub fn add(s: &mut Vec<f64>) {
    if s.len() >= 2 {
        let f1 = s.pop().unwrap();
        let f2 = s.pop().unwrap();
        let res = f1 + f2;
        s.push(res);
        println!("\top: {} + {}", f1, f2);
    }
}

pub fn sub(s: &mut Vec<f64>) {
    if s.len() >= 2 {
        let f1 = s.pop().unwrap();
        let f2 = s.pop().unwrap();
        let res = f1 - f2;
        s.push(res);
        println!("\top: {} - {}", f1, f2);
    }
}

pub fn sum(s: &mut Vec<f64>) {
    if s.len() >= 2 {
        let mut res: f64 = 0.0;
        for x in s.iter() {
            res += *x;
        }
        s.clear();
        s.push(res);
        println!("\top: sum")
    }
}

pub fn drop(s: &mut Vec<f64>) {
    if s.len() >= 1 {
        s.pop();
        println!("\top: drop");
    }
}

pub fn quit() {
    std::process::exit(0)
}
