/*
Ownership rules
1, Each value in Rust has a variable that is its Ownership
2, There can be only one owner at a time
3, When the owner goes out of scope, the value will be dropped.
 */

/*Example for Rule 1
fn main (){
    let s1 = String::from("RUST");
    let len = calculate_len(&s1);
    println!("Length of '{}' is {}", s1, len);
}
fn calculate_len(s: &String)->usize{
    s.len()
}
*/

/*Example for Rule 2
fn main (){
    let s1 = String::from("RUST");
    let s2 = s1;
    println!("{}",s1);//error occurs because the data is no longer in s1, it has moved to s2 in the previous line. s1 no longer owns the string.
}
*/

/*Example for Rule3
fn main (){
    let s1 = String::from("RUST");
    let len = calculate_len(&s1);
    println!("Length of '{}' is {}", s1, len);
}

fn printlost(s: &string){
    println!("{}", &s1); //error : can not find value in this scope
}

fn calculate_len(s: &String)->usize{
    s.len()
}
*/
