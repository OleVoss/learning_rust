use std::io;

fn main() {
    let mut user_input: String = String::new();

    println!("#### FACTORIAL FINDER ####\n");

    loop {
        user_input.clear();
        println!("C to cancel");
        println!("Enter a number:");
        io::stdin().read_line(&mut user_input).expect("Failed reading line!");

        if user_input.trim_end() == "C" || user_input.trim_end() == "c" {
            break;
        }
        let number: i32 = user_input.trim_end().parse().expect("Failed to parse integer!");
        println!("{}! is {}\n", number, find_factorial(number));
    }
}

fn find_factorial(number: i32) -> i32 {
    let mut factorial: i32 = 1;
    for f in 1..number+1 {
        factorial *= f;
    }

    return factorial;
}
