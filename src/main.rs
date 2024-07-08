mod commands;
use commands::commands::{Command};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let command = Command::new(args);
    command.print();
}
