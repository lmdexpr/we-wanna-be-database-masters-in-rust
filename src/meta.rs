pub fn execute(cmd: &str) -> Result<(), String> {
    match cmd {
        ".exit" => std::process::exit(0),
        _       => Err(format!("Unrecognized command '{}'", cmd)),
    }
}
