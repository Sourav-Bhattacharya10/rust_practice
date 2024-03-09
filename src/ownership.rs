pub fn ownership_test(){
    let s1 = String::from("Sourav");
    let s2 = s1;
    // println!("ownership file: {}", s1); // borrow of moved value: `s1`
    // value borrowed here after move
}

// ownership and functions
pub fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

// This function takes a String and returns one
pub fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
    // scope
    a_string  // a_string is returned and moves out to the calling function
}