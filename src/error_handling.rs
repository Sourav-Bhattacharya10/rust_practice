use std::fs::File;

pub fn option_test(){
    let fruits = vec!["banana", "apple", "coconut"];
    let first = fruits.get(0);
    let non_existent = fruits.get(99);
    println!("Error handling file: {:?} {:?}", first, non_existent);
}

pub fn result_test(){
    // let f = File::open("hello.txt");

    // let res = match f {
    //     Ok(file) => file,
    //     Err(error) => panic!("Can't open the file {:?}", error)
    // };
    // OR
    // let f = File::open("hello.txt").unwrap();
    // OR
    let f = File::open("hello.txt").expect("Can't open the file");
}