use std::io;
use std::io::Write;


pub fn get_answer(question: &str) -> Result<usize, &'static str> {
    let mut user_input: String = String::new();
    print!("{}", question);
    io::stdout().flush().expect("Failed to write line!");
    match io::stdin().read_line(&mut user_input) {
        Ok(input) => Ok(input),
        Err(_err) => Err("failed to read input"),
    }
}

pub fn get_integer_answer(question: &str) -> Result<i32, &'static str> {
    let mut user_input: String = String::new();
    print!("{}", question);
    io::stdout().flush().expect("Failed to write line!");
    io::stdin().read_line(&mut user_input).expect("Failed reading line!");
    match user_input.trim().parse::<i32>() {
        Ok(number) => Ok(number),
        Err(_error) => Err("Failed to read parse i32")
    }
}