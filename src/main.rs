use std::env;
use text_io::read;
mod ops;

fn main() {
    banner();

    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        ops::help();
        ops::quit();
    }

    let mut stack: Vec<f64> = Vec::new();
    loop {
        let line: String = read!("{}\n");
        match &line[..] {
            "+" | "add" => ops::add(&mut stack),
            "-" | "sub" => ops::sub(&mut stack),
            "*" | "mul" => ops::mul(&mut stack),
            "/" | "div" => ops::div(&mut stack),
            "sum" => ops::sum(&mut stack),
            "mean" => ops::mean(&mut stack),
            "swap" => ops::swap(&mut stack),
            "c" | "clear" => ops::clear(&mut stack),
            "d" | "drop" => ops::drop(&mut stack),
            "q" | "quit" => ops::quit(),
            "?" | "h" | "help" => ops::help(),
            _ => {
                let f = line.parse::<f64>().unwrap();
                stack.push(f);
            }
        }
        display_stack(&mut stack)
    }
}

fn banner() {
    println!("Rust RPN Calculator\n")
}

fn display_stack(s: &mut Vec<f64>) {
    println!("\t== STACK ==");
    for item in s {
        println!("\t{}", item);
    }
}
