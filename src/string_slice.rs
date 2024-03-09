pub fn reference_slice(){
    let s = String::from("hello world");

    let hello = &s[0..5]; // [..5]
    let world = &s[6..11]; // [6..]
    
    println!("Value slice: {} {}", hello, world);
}

pub fn first_word(s: &str) -> &str { // deref coercion
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
