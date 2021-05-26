pub fn add(s: &mut Vec<f64>) {
    if s.len() >= 2 {
        let f1 = s.pop();
        let f2 = s.pop();
        let res = f1.unwrap() + f2.unwrap();
        s.push(res)
    }
}
