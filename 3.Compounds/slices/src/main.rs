fn main() {

    let mut arr = [1,2,3,4,5];
    let slice = & mut arr[1..3];

    // printing slice with the debug format only

    println!("{:?}", slice);

    slice[0] = 6;
    slice[1] = 7;

    println!("{:?}", arr)

}
