use std::io;
use std::io::{BufRead, Write};

use password_input_validation::password_validator::PasswordValidator;

fn main() {
    write!(io::stdout(), "Please type a password: ").expect("failed to write output");
    io::stdout().flush().expect("failed to flush prompt");

    let mut password = String::new();
    io::stdin()
        .lock()
        .read_line(&mut password)
        .expect("failed to read input");

    let validator = PasswordValidator {};
    let (status, mut err_list) = validator.validate(&password);
    if status {
        println!("OK");
    } else {
        while let Some(err) = err_list.pop() {
            println!("{err}");
        }
    }
}
