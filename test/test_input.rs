use std::io::{self, Write};


fn main(){
    print!("Enter your name : ");
    io::stdout().flush().expect("Error");

    let mut name: String = String::new();
    io::stdin().read_line(&mut name).expect("Eror");

    print!("Hello {} !", name);
}