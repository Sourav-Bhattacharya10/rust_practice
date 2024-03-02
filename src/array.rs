pub fn test_array(){
    let arr: [u8; 3] = [1, 2, 3];
    for i in arr {
        print!("{} ", i);
    }

    // using {:?} format specifier which implements Debug trait
    println!("\nArray : {:?}", arr);
}