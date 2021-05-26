use text_io::read;
mod ops;

fn main() {
    banner();

    let mut stack: Vec<f64> = Vec::new();
    loop {
        let line: String = read!("{}\n");

        match &line[..] {
            "+" | "add" => ops::add(&mut stack),
            "-" | "sub" => ops::sub(&mut stack),
            "sum" => ops::sum(&mut stack),
            "q" | "quit" => ops::quit(),
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
