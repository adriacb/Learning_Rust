use std::io;

fn main() {
    let mut input = String::new();  // init empty mutable string
    
    io::stdin().read_line(&mut input).expect("failed to read line"); // creating a mutable reference to this input variable (accés the input variable)
    // expect caches any errors
    println!("{}", input);
}
