pub fn add(s: &mut Vec<f64>) {
    if s.len() >= 2 {
        let f1 = s.pop().unwrap();
        let f2 = s.pop().unwrap();
        let res = f1 + f2;
        s.push(res);
        println!("\t\top: {} + {}", f1, f2);
    }
}

pub fn sub(s: &mut Vec<f64>) {
    if s.len() >= 2 {
        let f1 = s.pop().unwrap();
        let f2 = s.pop().unwrap();
        let res = f1 - f2;
        s.push(res);
        println!("\t\top: {} - {}", f1, f2);
    }
}

pub fn mul(s: &mut Vec<f64>) {
    if s.len() >= 2 {
        let f1 = s.pop().unwrap();
        let f2 = s.pop().unwrap();
        let res = f1 * f2;
        s.push(res);
        println!("\t\top: {} * {}", f1, f2);
    }
}

pub fn div(s: &mut Vec<f64>) {
    if s.len() >= 2 {
        let f1 = s.pop().unwrap();
        let f2 = s.pop().unwrap();
        let res = f1 / f2;
        s.push(res);
        println!("\t\top: {} / {}", f1, f2);
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
        println!("\t\top: sum")
    }
}

pub fn modulo(s: &mut Vec<f64>) {
    if s.len() >= 2 {
        let f1 = s.pop().unwrap();
        let f2 = s.pop().unwrap();
        let res = f1 % f2;
        s.push(res);
        println!("\t\top: {} % {}", f1, f2);
    }
}

pub fn mean(s: &mut Vec<f64>) {
    if s.len() >= 1 {
        let mut res: f64 = 0.0;
        for x in s.iter() {
            res += *x;
        }
        let div: f64 = s.len() as f64;
        res = res / div;
        s.clear();
        s.push(res);
        println!("\t\top: mean");
    }
}

pub fn swap(s: &mut Vec<f64>) {
    if s.len() >= 2 {
        let f1 = s.pop().unwrap();
        let f2 = s.pop().unwrap();
        s.push(f1);
        s.push(f2);
        println!("\t\top: swap");
    }
}

pub fn clear(s: &mut Vec<f64>) {
    s.clear();
    println!("\t\top: clear")
}

pub fn drop(s: &mut Vec<f64>) {
    if s.len() >= 1 {
        let val = s.pop().unwrap();
        println!("\t\top: drop {}", val);
    }
}

pub fn quit() {
    std::process::exit(0)
}

pub fn help() {
    println!(
        "
== Help ==

+, add     : add the last 2 valutes of the stack
-, sub     : substract the last 2 values of the stack
*, mul     : multiply the last 2 values of the stack
/, div     : divide the last 2 values of the stack
sum        : sum the stack
mean       : calculate the mean value of the stack
swap       : swap the last 2 values of the stack
c, clear   : clear the stack
d, drop    : drop the last value of the stack
q, quit    : quit the program
?, h, help : show this help

==========
    "
    )
}
