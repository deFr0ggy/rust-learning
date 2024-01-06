fn main() {
    // str - can not be modified
    // String - can be modified
    // &st - subset of string

    let name = "Frog Man"; // immutable thats why
    name.push_str("test") // this will not work
    println!("{}", name)
    let mut name : String = String::new();
    name.push_str("test")
    println!("{}", name)
    let name : String = "FrogMan".tostring();
    let name : String = String::from("FrogMan");

}
