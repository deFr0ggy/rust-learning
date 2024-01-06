use std::io;

fn main() {

    println!("Enter your name");

    let mut name : String = String::new();

    io::stdin().read_line(&mut name);

    let enter = "End It?";

    println!("Your name is {}. {}", name.trim_end(), enter);
    
}
