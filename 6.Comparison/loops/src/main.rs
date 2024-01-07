fn main() {

// loop

let mut count = 0;
loop {
  count += 1;
  println!("Count: {}", count);
  if count == 10 {
    break;
  }
} 

// while loop

let mut count = 0;
while count < 10 {
  count += 1;
  println!("Count: {}", count);
} 

// for loop

for i in 1..=10 {
    println!("Count: {}", i);
  }
  
  let numbers = vec![1, 2, 3, 4, 5];
  
  for num in numbers {
    println!("Number: {}", num);
  } 
  

}
