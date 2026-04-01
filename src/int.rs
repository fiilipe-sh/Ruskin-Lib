use std::io::{self, Write};
const RESET: &str = "\x1b[0m";
const RED: &str = "\x1b[31m";
const GRAY: &str = "\x1b[90m";

fn prompt_int32(prompt: &str) -> i32{
    let mut input_user = String::new();

    loop{
        input_user.clear();
        println!("{}[only whole numbers]{}",GRAY,RESET);
        print!("{} ",prompt);
        //forcing the user's text to appear on the screen
        io::stdout().flush().expect("Error displaying your buffer.");
        //Read the line that the user typed and saved in the variable.
        io::stdin().read_line(&mut input_user).expect("Error in text input.");

        match input_user.trim().parse::<i32>(){
            Ok(n) => return n,
            Err(_) => {
                println!(" {}->{} {} {}is not a valid number! Try again.{}",RED,RESET,input_user.trim(),RED,RESET);
            }
        }
    };
}



