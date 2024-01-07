fn main() {
   
    let x = 3;
    match x {
      0 => println!("x is zero"),
      1 | 2 => println!("x is one or two"), 
      _ => println!("x is something else"),
    } 
}
