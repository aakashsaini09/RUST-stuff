fn main() {
    conditionalsta();
    stack_heap();
    ownershipfunction()
}


fn strInt(){
    // number
    println!("I'm a Rustacean!");
    let x: i32 = 20;
    println!("The value of x is: {}", x*2);
    for i in 0..x {
        println!("i = {}", i);
    }
    // string
    let str = String::from("Hello, world!");
    for c in str.chars() {
            println!("check = {}\n", c);
        }
        let b = str.to_lowercase();
        println!("str = {}", str);
    let greeting = String::from("RUST IS COOL");
    let char1 = str.chars().nth(1);
    match char1 {
        Some(c) => println!("char1 = {}", c),
        None => println!("No character at index 0"),
    }
    print!("str: {}", char1.unwrap());
}


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
    let sentence = String::from("Hello, world!");
    let first_word = get_first_word(sentence);
    print!("first word = {}", first_word);
    fn get_first_word(sentence: String) -> String{
        let mut ans = String::from("");
        for char in sentence.chars(){
            ans.push_str(char.to_string().as_str());
            if char == ' '{
                break;
            }
        }
        return ans;
    }
}
fn stack_heap() {
   let mut str = String::from("Hello, world!");
    print!("before update: {}\n", str);
    print!("Capacity: {}, Length: {}, pointer: {:p}", str.capacity(), str.len(), str.as_ptr());

    str.push_str("second time");
    print!("After 1st update: {}\n", str);
    print!("Capacity: {}, Length: {}, pointer: {:p}\n", str.capacity(), str.len(), str.as_ptr());

    for _ in  0..10 {
        str.push_str("second time");
    print!("Capacity: {}, Length: {}, pointer: {:p}\n", str.capacity(), str.len(), str.as_ptr());
    }
}



fn ownershipfunction(){
    let s1 = String::from("hello");
    print!("s1 = {}\n", s1);
    let s2 = s1;
    println!("s1 = {}", s2);
}
