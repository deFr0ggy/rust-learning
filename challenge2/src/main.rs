use std::io;

fn main() {
    
    let mut value1 : String = String::new();

    io::stdin().read_line(&mut value1);

    let x : i32 = value1.trim().parse().expect("Entry was not an integer");

    let mut value2 : String = String::new();

    io::stdin().read_line(&mut value2);

    let y : i32 = value2.trim().parse().expect("Entry was not an integer");

    println!("Multiplication: {}", x*y);
    println!("Multiplication: {}", x/y);
    println!("Multiplication: {}", x-y);
    println!("Multiplication: {}", x+y);
    println!("Multiplication: {}", x%y);

}
