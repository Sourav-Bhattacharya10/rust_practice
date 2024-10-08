// use crate::struct_file::Car;
// use crate::classic_struct_file2::Person;
// use crate::tuple_struct::Point2D;
// use crate::enums_file::CardinalDirections;
// use crate::generic_enum_file::Options;
// use crate::classic_struct_file3::Rectangle;
// use std::env;
// use crate::trait_file::{NewsArticle, Tweet, Summary, notify, returns_newsarticle_summarizable};
use trait_object::{Button, Screen};

// mod print;
// mod array;
// mod functions;
// mod classic_struct_file;
// mod classic_struct_file2;
// mod tuple_struct;
// mod enums_file;
// mod generic_enum_file;
// mod control_flow;
// mod loop_file;
// mod error_handling;
// mod ownership;
// mod reference_borrow;
// mod string_slice;
// mod classic_struct_file3;
// mod rand_test;
// mod vector_test;
// mod trait_file;
mod trait_object;

fn main() {
    // // print.rs
    // print::print_number(10);
    // print::print_message("Sourav");
    // print::print_bases();

    // // array.rs
    // array::test_array();

    // // function.rs
    // let flag = functions::return_bool();
    // println!("Bool {}", flag);
    // let greeting = functions::greet_string(String::from("Deviac"));
    // println!("Greeting message : {}", greeting);

    // // classic_struct_file.rs
    // let car1 = classic_struct_file::Car {
    //     make: String::from("Ford"),
    //     model: String::from("Mustang"),
    //     year: 1967
    // };
    // println!("Car Details {make} {model} {year}", make=car1.make, model=car1.model, year=car1.year);

    // // classic_struct_file2.rs
    // let person1 = Person {
    //     name: String::from("Priya"),
    //     like_oranges: true
    // };
    // let does_like_oranges = if person1.like_oranges { "like oranges" } else { "does not like oranges" };
    // println!("Person Details : {0} {1}", person1.name, does_like_oranges);

    // // tuple_struct.rs
    // let origin = Point2D(10, 20);
    // let Point2D(i, j) = origin;
    // println!("Point coordinates are : {:?}", origin);
    // println!("After destructuring, point coordinates are : {} {}", i, j);

    // // enums_file.rs
    // let direction1 = CardinalDirections::North;
    // let direction2 = CardinalDirections::South(String::from("South"));
    // println!("Directions {} {}", direction1, direction2);

    // // generic_enum_file.rs
    // let something = Options::<i32>::Some(1);
    // let none = Options::<i32>::None;
    // println!("Generic enum : {}, {}", something, none);

    // // control_flow.rs
    // control_flow::match_test(something);

    // // loop_file.rs
    // loop_file::for_test();

    // // error_handling.rs
    // // error_handling::option_test();

    // // error_handling::result_test();

    // // ownership.rs
    // ownership::ownership_test();

    // let s1 = String::from("SB");
    // ownership::takes_ownership(s1);
    // // println!("Ownership and Functions : {}", s1); // error, s1 is moved in the above statement

    // let s2 = String::from("hello");     // s2 comes into scope
    // let s3 = ownership::takes_and_gives_back(s2);  // s2 is moved into
    //                                     // takes_and_gives_back, which also
    //                                     // moves its return value into s3
    // println!("Ownership and Function returns : {}", s3);

    // // reference_borrow.rs
    // let s1 = String::from("hello");
    // let len = reference_borrow::calculate_length(&s1);
    // println!("The length of '{}' is {}.", s1, len);

    // let mut mut_str = String::from("hello");
    // reference_borrow::change(&mut mut_str);
    // println!("Mutable reference: {}", mut_str);

    // // let mut s = String::from("hello");

    // // let r1 = &s; // no problem
    // // let r2 = &s; // no problem
    // // let r3 = &mut s; // BIG PROBLEM

    // // println!("{}, {}, and {}", r1, r2, r3);

    // // let mut s = String::from("hello");

    // // let r1 = &s; // no problem
    // // let r2 = &s; // no problem
    // // println!("{} and {}", r1, r2);
    // // // variables r1 and r2 will not be used after this point

    // // let r3 = &mut s; // no problem
    // // println!("{}", r3);

    // // string_slice.rs
    // string_slice::reference_slice();

    // let mut s = String::from("hello world");
    // let word = string_slice::first_word(&s);

    // // s.clear(); // error!

    // println!("the first word is: {}", word);

    // // classic_struct_file3.rs
    // let rect1 = Rectangle {
    //     width: 30,
    //     height: 40
    // };

    // let area = classic_struct_file3::area_rectangle(&rect1);
    // println!("Area of the Rectangle : {}, {:?}", area, rect1);

    // // method syntax
    // let area2 = rect1.area_of_rectangle();
    // println!("Area of the Rectangle using Method syntax : {}, {:?}", area2, rect1);

    // // rand_test.rs
    // let sec_num = rand_test::generate_random_number(1, 10);
    // println!("Generated random number: {sec_num}");

    // // vector_test.rs
    // vector_test::updating_vectors();
    // vector_test::reading_vectors();
    // vector_test::iterating_and_mutating_vectors();

    // let args: Vec<_> = env::args().collect();
    // if args.len() > 0 {
    //     println!("Args : {:?}", args);
    // }

    // let tweet = Tweet {
    //     username: String::from("horse_ebooks"),
    //     content: String::from(
    //         "of course, as you probably already know, people",
    //     ),
    //     reply: false,
    //     retweet: false,
    // };

    // println!("1 new tweet: {}", tweet.summarize());
    // notify(&tweet);

    // let article = NewsArticle {
    //     headline: String::from("Penguins win the Stanley Cup Championship!"),
    //     location: String::from("Pittsburgh, PA, USA"),
    //     author: String::from("Iceburgh"),
    //     content: String::from(
    //         "The Pittsburgh Penguins once again are the best \
    //          hockey team in the NHL.",
    //     ),
    // };

    // println!("New article available! {}", article.summarize());

    // println!("Another article! {}", returns_newsarticle_summarizable().summarize());

    let screen = Screen {
        components: vec![Box::new(Button {
            width: 50,
            height: 10,
            label: String::from("OK"),
        })],
    };

    screen.run();
}
