use std::io::{self, Write};


fn prompt_string(prompt: &str) -> String{

    print!("{} ",prompt);

    //forcing the user's text to appear on the screen
    io::stdout().flush().expect("Error displaying your buffer.");

    let mut input_user = String::new();

    //Read the line that the user typed and saved in the variable.
    io::stdin().read_line(&mut input_user).expect("Error in text input.");

    //removing the enter key that remained in the buffer
    let response = input_user.trim();

    return response.to_string();
}
