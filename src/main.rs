use std::io::{self, BufRead, Write};
use meval::{eval_str_with_context, Context};

fn main() {
    let mut context = Context::new();
    context.func("square", square);

    if std::env::args().len() > 1 {
        let args: Vec<String> = std::env::args().collect();
        let mut expr = String::new();
        if args.len() == 2 && (args[1] == "help" || args[1] == "h") {
            print_help();
            return;
        }
        for i in 1..args.len() {
            expr.push_str(&args[i]);
            expr.push(' ');
        }
        match eval_str_with_context(&expr, &context) {
            Ok(result) => println!("= {}", result),
            Err(_) => eprintln!("Invalid expression"),
        }
        return;
    }
    
    loop {
        print!("> ");
        let _ = io::stdout().flush();
        let line = io::stdin().lock().lines().next().unwrap().unwrap(); 
        if line == "" {
            continue;
        }
        if line == "exit" || line == "quit" || line == "q" {
            break;
        }
        if line == "help" || line == "h" {
            print_help();
            continue;
        }
        match eval_str_with_context(&line, &context) {
            Ok(result) => println!("= {}", result),
            Err(_) => {eprintln!("Invalid expression")},
        }
    }
}

fn square(x: f64) -> f64 {
    x * x
}

fn print_help() {
    println!("Available functions:");
    println!(" square, sqrt, abs, exp, ln, floor, ceil, round, signum, min(x, y, ...), max(x, y, ...)");
    println!(" sin, cos, tan, asin, acos, atan, atan2, sinh, cosh, tanh, asinh, acosh, atanh");
    println!("Available constants:");
    println!(" pi, e");
}