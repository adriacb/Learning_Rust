fn main() {
    let x = 2;          // implicitly an integer
    println!("{}", x);
    let x: i32 = 2;     // assigned integer (default)
    // i8, i16, i32, i64, i128
    println!("{}", x);
    let x: u32 = 2;     // unsigned integer (default)
    // u8, u16, u32, u64, u128
    println!("{}", x);

    let floating_point: f32 = 10.92;
    // f32(default), f64
    println!("{}", floating_point);

    let true_or_false: bool = true;
    // false, 1, 0
    println!("{}", true_or_false);
    let letter: char = 'C';
    // idk with single quotes
    println!("{}", letter);

    let tup: (i32, bool, char) = (1, true, 's');
    println!("{}", tup.0);

    let mut arr: [i32; 5] = [1, 2, 3, 5, 5];
    arr[3] = 0;
    println!("{}", arr[3]);    
}

// https://www.youtube.com/watch?v=t047Hseyj_k&list=PLzMcBGfZo4-nyLTlSRBvo0zjSnCnqjHYQ&index=4
// 16:37