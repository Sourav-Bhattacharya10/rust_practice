pub fn loop_test(){
    loop {
        println!("I will keep running forever until I encounter break statement");
        break;
    }
}

pub fn while_test(){
    let mut num = 3;

    while num > 0 {
        println!("Value of num: {}", num);
        num -= 1;
    }
}

pub fn for_test(){
    for item in 0..5{
        println!("For test : {}", item);
    }
}