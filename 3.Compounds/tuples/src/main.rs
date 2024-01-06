fn main() {
    
    let mydetails = ("Frog Man", "Twitter", 007);

    print!("This guy named {} from {} is {}", mydetails.0, mydetails.1, mydetails.2);

    let (mydetails_a, mydetails_b, mydetails_c) = mydetails;

    print!("{} {} {}", mydetails_a, mydetails_b, mydetails_c);

}
