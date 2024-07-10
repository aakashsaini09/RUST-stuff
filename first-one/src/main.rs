fn main() {
    conditionalsta()
}
// fn strInt(){
//     // number
//     println!("I'm a Rustacean!");
//     let x: i32 = 20;
//     println!("The value of x is: {}", x*2);
//     for i in 0..x {
//         println!("i = {}", i);
//     }
//     // string
//     for c in str.chars() {
//             println!("check = {}\n", c);
//         }
//         let b = str.to_lowercase();
//         prinstln!("str = {}", str[0..2]);
//     let greeting = String::from("RUST IS COOL");
//     let char1 = str.chars().nth(1);
//     match char1 {
//         Some(c) => println!("char1 = {}", c),
//         None => println!("No character at index 0"),
//     }
//     print!("str: {}", char1.unwrap());
// }


fn conditionalsta(){
    // conditional statements
    let is_even = false;
    if is_even{
        println!("It is even");
    }else if !is_even{
        println!("The nuber is odd")
    }else{
        println!("The number is not a number");
    }
    for i in 0..10 {
        print!("i = {}", i);
    }
}
