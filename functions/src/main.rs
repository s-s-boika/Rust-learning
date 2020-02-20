fn main() {
    another_function(5);
    println!("Returned = {}", return_function(5));
}


fn another_function(x: i32) {
    println!("The value of the X is: {}", x);
}


fn return_function(x: i32) -> i32 {
    let b = 123;
    //return x+b;
    x+b
}
