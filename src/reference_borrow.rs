// reference
pub fn calculate_length(s: &String) -> usize {
    s.len()
}

// mutable reference
pub fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// // dangle reference
// pub fn dangle() -> &String { // dangle returns a reference to a String

//     let s = String::from("hello"); // s is a new String

//     &s // we return a reference to the String, s
// } // Here, s goes out of scope, and is dropped. Its memory goes away.
//   // Danger!