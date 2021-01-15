use std::io::{stdin, stdout, Write};

enum StatementType {
    StatementInsert,
    StatementSelect,
}

struct Statement {
    s_type: StatementType,
}

fn execute_meta(cmd: &str) -> Result<(), String> {
    match cmd {
        ".exit" => std::process::exit(0),
        _       => Err(format!("Unrecognized command '{}'", cmd)),
    }
}

fn parse(raw: &str) -> Result<Statement, String> {
    match raw {
        raw if raw.starts_with("insert") =>
            Ok(Statement { s_type : StatementType::StatementInsert }),
        "select" => Ok(Statement { s_type : StatementType::StatementSelect }),
        raw      => Err(format!("Unrecognized keyword at start of '{}'", raw)),
    }
}

fn execute(statement: Statement) {
    match statement.s_type {
        StatementType::StatementInsert => println!("This is where we would do an insert."),
        StatementType::StatementSelect => println!("This is where we would do an select."),
    }

    println!("Executed.")
}

fn interpreter() {
    print!("db > ");
    stdout().flush().unwrap();

    let line = {
        let mut line = String::new();
        stdin().read_line(&mut line).unwrap();
        &*line.trim().to_owned()
    };

    if line.starts_with(".") {
        execute_meta(line)
    } else {
        parse(line).map(execute)
    }
    .unwrap_or_else(|msg| { println!("{}", msg) })
}

fn main() {
    loop { interpreter() }
}
