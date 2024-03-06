pub fn print_number(num: u32){
    println!("Number: {}", num);
}

pub fn print_message(word: &str){
    println!("String : {}", word);
}

// Placeholder traits
pub fn print_bases(){
    println!("Binary {0:b} Hex {0:x} Octal {0:o}", 10);
}