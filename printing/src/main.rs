fn main() {
    // arrays
    let numbers: [i32; 5] = [1,2,3,4,5];
    println!("number array ={:?}", numbers);
    let fruits: [&str; 3] = ["apple","banana","orange"];
    println!("fruits array : {:?}", fruits);
    println!("fruits array : {}", fruits[0]);
    println!("fruits array : {}", fruits[1]);
    println!("fruits array : {}", fruits[2]);

    //tuples
    let human : (String, i8, bool)= ("ishaan".to_string(), 1, true);//if you remove (String, i8, bool) from this line, you wont need to define .to_string()
    //also switching from String to &str fixes removes the need for to_string()
    println!("this is a tuple: {:?}", human);


    //slices
    let number_slices:&[i32] = &[1,2,3,4,5];
    println!("number slice : {:?}", number_slices);

    let animals_slice:&[&str] = &["lion","elephant","crocodile"];
    println!("animal slice : {:?}", animals_slice);

    let book_slices:&[&String] = &[&"IT".to_string(), &"Harry Potter".to_string(), &"Zen".to_string()];
    println!("book slice: {:?}", book_slices);


    //String vs String Slices
    //Strings : [growable, mutable, owned string type] - Stored on the heap
    let mut stone_cold: String = String::from("He"); //stone_cold is stored on the heap memory
    println!("Stone cold says: {}", stone_cold);
    stone_cold.push_str("llo");
    println!("Stone cold says: {}", stone_cold);
    //String Slice
    let string1: String = String::from("hello world");
    let slice: &str = &string1[0..5];
    println!("Slice value = {}", slice);
}
