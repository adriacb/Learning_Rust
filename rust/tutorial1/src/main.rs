fn main() {
    let mut x = 4; // WE HAVE TO DEFINE THE VARIABLE AS mut
    println!("x is {}", x);
    x = 5;                      // ERROR: cannot assign twice to immutable variable
    // BY DEFAULT IN RUST ALL VARIABLES ARE
    // IMMUTABLE
    println!("x is {}", x);

    let x = 5; // DO WE NEED TO MUTATE THIS VARIABLE?
    println!("x is {}", x);


    {   // THIS IS A NEW SCOPE, IT WON'T CHANGE ANYTHING IN THE OUTSIDE
        let x = 10; 
        println!("x is {}", x);        
    }

    let x = x + 1;
    println!("x is {}", x);

    const SECONDS_IN_MINUTE: u32 = 60;

    println!("CONST {}", SECONDS_IN_MINUTE);

}
