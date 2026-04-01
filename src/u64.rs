use std::io::{self, Write};

fn prompt_u64(prompt: &str) -> u64{

     let mut input_user = String::new();
      loop{
          input_user.clear();

          print!("{} ",prompt);

          //forcing the user's text to appear on the screen
          io::stdout().flush().expect("Error displaying your buffer.");

          //Read the line that the user typed and saved in the variable.
          io::stdin().read_line(&mut input_user).expect("Error in text input.");

   match input_user.trim().parse::<u64>(){
     
      Ok(n) => return n,
      Err(_) => {
           println!("{} is not a valid number! Try again.", input_user.trim());
      }
    }
  };
}


