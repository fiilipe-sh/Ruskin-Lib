use std::io::{self, Write};


fn prompt_int32(prompt: i32) -> i32{



    print!("{} ",prompt);

    //forcing the user's text to appear on the screen
    io::stdout().flush().expect("Error displaying your buffer.");

    let mut input_user = String::new();

    //Read the line that the user typed and saved in the variable.
    io::stdin().read_line(&mut input_user).expect("Error in text input.");

    //Check if it is an integer.
    if let Err(error) = input_user.trim().parse::<i32>(){
        println!("{}",error);

    }

    return input_user;
}
