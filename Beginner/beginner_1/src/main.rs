mod user_input_controller;
mod tasks;


fn main() {
    let mut user_input: String = String::new();

    println!("#### BEGINNER_1 ####\n");

    loop {
        user_input.clear();
        println!("The following tasks are available:");
        println!("1) finding the factorial");
        println!("2) test something");

        let task_to_execute: i32 = user_input_controller::get_integer_answer("Execute Task: ").unwrap();

        match task_to_execute {
            1 => tasks::find_factorial(),
            2 => tasks::testing(),
            _ => {}
        }
    }
}
