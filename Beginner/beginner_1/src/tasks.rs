use crate::user_input_controller;


pub fn find_factorial() {
    let mut factorial: i32 = user_input_controller::get_integer_answer("Enter an integer: ").unwrap();
    let original_number: i32 = factorial.clone();
    for f in 1..original_number {
        factorial *= f;
    }
    println!("The factorial of {} is {} \n", original_number, factorial);
}

pub fn testing() {
    match user_input_controller::get_answer("Enter shit:") {
        Ok(input) => println!("Ok"),
        Err(err) => println!("{}", err),
    }
}