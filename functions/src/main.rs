fn hello_world(){
    println!("Hello world !");
}
//define the function first, call it later: called hoisting

fn tell_height(height: u32){
    println!("My height is {}", height);
}
fn human_id(name: &str, age: u32, height: f32){
    println!("My name is {}, I am {} years old and {}cm tall", name, age, height);
}

fn main() {
    hello_world();
    tell_height(174);
    human_id("ishaan", 22, 174.0);
    let _X:i32 ={
        let price:i32=5;
        let qty:i32=10;
        price * qty //dont need to put a ; or do return price * qty; here, even though you could. rust feature
    };
    println!("result is {}", _X);
    let y:i32= add(4,6);
    println!("add function result = {}", y);

    //calling the BMI function
    let height = 1.74;
    let weight = 55.0;
    println!("BMI of said individual is : {}", calculate_bmi(weight, height));
}

fn add(a:i32, b:i32) -> i32{
    a+b
}
//any variable declared outside the main fn should be declared using the const keyword
// constant should have uppercase snake name like so
/*
const X = {

};
*/
//expressinos and statements
// an expression is anything that returns a value : Example - any number, any bool, any operation like addition, if conditions
// a statement is anything that does not return a value : Example -


/*
BMI = Weight (kg)/Height(m)^2
 */

 //BMI function
fn calculate_bmi(weight_kg:f64, height_m:f64)->f64{
    weight_kg/(height_m*height_m)
}
