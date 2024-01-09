fn main() {
    test_one();
    test_one();

    add_numbers(1, 3);

    /* STATEMENTS */
    let y = 20;


    /* EXPRESSION */
    let number = {
        let x = 3;
        x + 1       // WE DONT HVE SEMICOLON HERE
    };
    println!("{}", number);


    let result = add_numbers2(2, 3);
    println!("{}", result);

}

fn test_one() {     // IT DOESN'T MATTER THE ORDER
    println!("Test one has been called...");
}

fn add_numbers(x: i32, y: i32) {
    println!("The sum is {}", x + y);
}

fn add_numbers2(x: i32, y: i32) -> i32 { // the return operator in RUST
    x + y   //WITHOUT SEMICOLON!!!
    // return x + y (it will work also)
}


