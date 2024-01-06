#![allow(unused)]

fn main() {
    let x = 1;

    // Scoping is the below

    {

        let y = 2;
        // we can not use x here. So x has to be defined withhin the braces

    }


    // Shadowing is below i.e. redefine the variable with same name.

    let x = 1;
    let x = 2;

// turning off errors at the top

}
