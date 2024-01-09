use std::io;

fn main() {
    /*let x: u8 = 255;    // 0 - 255
    let y: i8 = 10;     // -128 - 127

    let z = x + y;      // ^ no implementation for `u8 + i8`
    println!("{}", z);

    // we can only add variables with the SAME TYPE (exactly the same f32 with f32 ok but i8 with i64 NO)
    */

    /*let x: u8 = 255;   
    let y: u8 = 1;     
    let z = x + y;
    println!("{}", z);
    // attempt to compute `u8::MAX + 1_u8`, which would overflow
    // ideally we want to cast these values with a larger type (e.g. u16)*/

    let x: u8 = 255;   
    let y: u8 = 10;     
    let z = x / y;      // 25 as result as is a u8 value, the decimal part is not showed, THE SAME HAPPENS WITH MULTIPLICATION
    println!("{}", z);

    let x: u8 = 255;   
    let y: u8 = 10;     
    let z = x % y;      // 5
    println!("{}", z);


    /* CAST AND TYPES */
    let x = 255.0f32;   
    let y = 10.0f32;     
    let z = x % y;      
    println!("{}", z); 


    let x = 127_i8;   
    let y = 10_i8;     
    let z = x % y;      
    println!("{}", z);   

    let x = 127_000 as i64;   
    let y = 10_i32;     
    let z = x / (y as i64);      
    println!("{}", z);         

    let x = (i32::MAX as i64) + 1;
    let y = 10_i32;     
    let z = (x as i32) / y;      // WE HAVE AN OVERFLOW EVEN GETTING A NEGATIVE VALUE IT IS NOT CATCHED BY THE COMPILER
    println!("{}", z);   
            
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("NOT CORRECT");

    let int_input: i64 = input.trim().parse().unwrap(); // trim removes the "\n" and them parse

    println!("OUTPUT: {}", int_input + 2); 
}
