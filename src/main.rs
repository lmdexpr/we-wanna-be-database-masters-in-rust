use std::io::{stdin, stdout, Write};

mod meta;
mod statement;

fn interpreter() {
    print!("db > ");
    stdout().flush().unwrap();

    let line = {
        let mut line = String::new();
        stdin().read_line(&mut line).unwrap();
        &*line.trim().to_owned()
    };

    if line.starts_with(".") {
        meta::execute(line)
    } else {
        statement::execute(line)
    }
    .unwrap_or_else(|msg| { println!("{}", msg) })
}

fn main() {
    loop { interpreter() }
}
