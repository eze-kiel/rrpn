use std::env;
use std::io::{self, Write};
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
        print!("rrpn> ");
        io::stdout().flush().unwrap();
        let line: String = read!("{}\n");
        let split = line.split(" ");
        let vec: Vec<&str> = split.collect();

        for item in vec.iter() {
            match *item {
                "+" | "add" => ops::add(&mut stack),
                "-" | "sub" => ops::sub(&mut stack),
                "*" | "mul" => ops::mul(&mut stack),
                "/" | "div" => ops::div(&mut stack),
                "^" | "pow" => ops::pow(&mut stack),
                "%" | "mod" => ops::modulo(&mut stack),
                "sin" => ops::sin(&mut stack),
                "cos" => ops::cos(&mut stack),
                "sum" => ops::sum(&mut stack),
                "mean" => ops::mean(&mut stack),
                "swap" => ops::swap(&mut stack),
                "c" | "clear" => ops::clear(&mut stack),
                "d" | "drop" => ops::drop(&mut stack),
                "q" | "quit" => ops::quit(),
                "?" | "h" | "help" => ops::help(),
                _ => {
                    let f = item.parse::<f64>();
                    let _f = match f {
                        Ok(float) => stack.push(float),
                        Err(_error) => println!("operation not recognised"),
                    };
                }
            }
        }
        display_stack(&mut stack)
    }
}

fn banner() {
    println!("Rust RPN Calculator\n")
}

fn display_stack(s: &mut Vec<f64>) {
    println!("\t\t== STACK ==");
    for item in s {
        println!("\t\t{}", item);
    }
}
