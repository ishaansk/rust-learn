/*
Refereces and Borrowing
Safety and Performance
Borrowing and references are powerful concepts

Understanding references
References: Enable you to borrow values without taking ownership.
Immutable Reference.
Mutable Reference
To create a reference, put a & before the variable name
 */

fn main(){
    let mut _x: i32 = 5;

    let _r: &mut i32 = &mut _x;

    *_r+=1;
    *_r-=3;

    println!("Value of _x : {}", _x);
    //println!("Value of _r : {}", _r); This throws an error because you can only either have one mutable reference or any number of immutable references.
    // In the above code line we are trying to borrow _x as an immutable reference while it is already borrowed as a mutable reference 
}
