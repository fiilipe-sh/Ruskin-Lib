use std::io::{self, Write};

fn prompt_string(prompt: &str) -> String{
    print!("{} ",prompt);
    io::stdout().flush().expect("Error displaying your buffer.");


    let mut input_user = String::new();
    io::stdin().read_line(&mut input_user).expect("Error in text input.");

    let response = input_user.trim();

    return response.to_string();
}



fn main(){

  let  resposta =  prompt_string("Preciso que informe sua idade: ");
  println!("{}", resposta);






}
