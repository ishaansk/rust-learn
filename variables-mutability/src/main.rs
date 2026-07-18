fn main() {
    println!("Hello, world!");
    let mut a: i32 = 5;
    println!("the value of a is {}",a);
    a = 10; //this line works only because a is mutable
    println!("the value of a is {}",a);
}
