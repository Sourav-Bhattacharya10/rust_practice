pub fn updating_vectors(){
    let mut v = Vec::new();
    println!("Vectors updated old: {:?}", v);

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    println!("Vectors updated new: {:?}", v);
}

pub fn reading_vectors(){
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }
}

pub fn iterating_and_mutating_vectors(){
    let mut v = vec![100, 32, 57];
    println!("Vectors old: {:?}", v);
    for i in &mut v {
        *i += 50;
    }

    println!("Vectors new: {:?}", v);
}