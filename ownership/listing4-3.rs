fn main() {
    let s = String::from("hello"); //s comes into scope

takes_ownership(s); //s's value moves into the function and
                    //so is no longer valid here

let x = 5; // x comes into scope

makes_copy(x); // x would move into the function,
               // but i32 is Copy, so it is okay
               // to still us x afterward
} 
// Here, x goes out of scope, then s. 
// However, because s's value was moved
// nothing special happens
//

fn takes_ownership(some_string: String) { // some_string comes 
                                          // into scope
    println!("{some_string}");
} // Here, some_Script goes out of scope and 
  // ''drop' is called. 
  // The backing memoery is freed. 

fn makes_copy(some_int: i32) { // some_int comes into scope
    println!("{some_int}");
} // Here, some_int goes out of scope. Nothing 
  // special happens. 

