pub enum StatementType {
    StatementInsert,
    StatementSelect,
}

pub struct Statement {
    s_type: StatementType,
}

pub fn execute(raw: &str) -> Result<(), String> {
    parse(raw).map(inner)
}

fn parse(raw: &str) -> Result<Statement, String> {
    match raw {
        raw if raw.starts_with("insert") =>
            Ok(Statement { s_type : StatementType::StatementInsert }),
        "select" => Ok(Statement { s_type : StatementType::StatementSelect }),
        raw      => Err(format!("Unrecognized keyword at start of '{}'", raw)),
    }
}

fn inner(statement: Statement) {
    match statement.s_type {
        StatementType::StatementInsert => println!("This is where we would do an insert."),
        StatementType::StatementSelect => println!("This is where we would do an select."),
    }

    println!("Executed.")
}

