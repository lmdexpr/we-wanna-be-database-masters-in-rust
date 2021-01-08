use std::io::{stdin, stdout, Write};

fn main() {
    loop {
        print!("db > ");
        stdout().flush().unwrap();

        let mut line = String::new();

        stdin().read_line(&mut line).unwrap();

        match &*line.trim() {
            ".exit" => std::process::exit(0),
            trimed => println!("Unrecognized command {}", trimed)
        }
    }
}
