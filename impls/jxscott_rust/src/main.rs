use std::io::{self, BufRead, Write};

fn read() -> String {
    print!("user> ");
    let _ = io::stdout().flush();
    let stdin = io::stdin();
    let mut iter = stdin.lock().lines();

    return iter.next().unwrap().unwrap().to_string();
}

fn eval(str: String) -> String {
    return str;
}

fn print(str: String) -> String {
    println!("{}", str);
    return str;
}

fn rep() {
    loop {
        print(eval(read()));
    }
}

fn main() {
    rep()
}
