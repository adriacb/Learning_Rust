fn main() {
    /*
        <
        >
        <=
        >=
        !=
        ==
     */

    let cond = 2 < 3;
    println!("{}", cond);

    /* WE CAN CONLY COMPARE SAME DATA TYPES! */

    /* 
        && (and)
        || (or)
        ! (not)    
    */

    let cond2 = cond && false;      // let cond2 = false || !cond;
    println!("{}", cond2);    


    let food = "cookie";

    if food == "cookie" {
        println!("I like cookies too!");
    }   // else if {}
    else {
        println!("FUCK YEAH")
    }

}
