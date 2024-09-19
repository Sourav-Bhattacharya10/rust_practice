// exercises
// use std::io;

// use exercises::collections_summary::emp_dept::{display_all_employees, Employee};

// use crate::exercises::control_flow_summary::temperature_conversion;
// use crate::exercises::control_flow_summary::fibonacci;
// use crate::exercises::collections_summary::vector_median_mode;
// use crate::exercises::collections_summary::emp_dept::{self, add_employee_to_dept, display_employees_in_a_dept_asc_ordered};
// use crate::exercises::data_structures::stack::Stack;
use exercises::data_structures::queue::Queue;

mod exercises;

fn main() {
    // // temperature_conversion
    // let f1: f32 = -40.0;
    // let c1 = temperature_conversion::convert_temp(f1, temperature_conversion::Temperature::Fahrenheit);
    // println!("{f1}F to {c1}C");

    // let c2: f32 = 0.0;
    // let f2 = temperature_conversion::convert_temp(c2, temperature_conversion::Temperature::Celsius);
    // println!("{c2}C to {f2}F");

    // // fibonacci
    // println!("Please input your nth term:");

    // let mut nth_term_str = String::new();

    // io::stdin()
    //     .read_line(&mut nth_term_str)
    //     .expect("Failed to read line");

    // let nth_term: u32 = match nth_term_str.trim_end().parse::<u32>() {
    //     Ok(n) => n,
    //     Err(e) => {
    //         println!("{:?}", e);
    //         0
    //     }
    // };

    // let fibo_value = fibonacci::generate_fibonacci(nth_term);

    // println!("The generated fibonacci value for the {nth_term}th term is {fibo_value}");

    // vector_median_mode
    // let int_list = vec![8, 3, 4, 1, 3, 2, 5, 5, 5];
    // let median = vector_median_mode::calculate_median(int_list.clone());
    // println!("The median for the {:?} vector is {}", int_list, median);

    // let mode = vector_median_mode::calculate_mode(&int_list);
    // println!("The mode for the {:?} vector is {}", int_list, mode);

    // emp_dept
    // println!("Welcome to Employee Management System!!");

    // let mut emps = Vec::<Employee>::new();

    // statically inserted employees
    // emps.push(Employee {emp_id: 1, name: String::from("Alfa"), dept_id: 1, dept: String::from("Engg")});
    // emps.push(Employee {emp_id: 2, name: String::from("Beta"), dept_id: 2, dept: String::from("Sales")});
    // emps.push(Employee {emp_id: 3, name: String::from("Charlie"), dept_id: 1, dept: String::from("Engg")});

    // get employees from user input
    // loop {
    //     println!("Please type one of the options:");
    //     println!("1. To add a new employee with Department Name, type - Add Alfa to Engg/Sales/Marketing");
    //     println!("2. To display employees in a department, type - Display Engg/Sales/Marketing employees");
    //     println!("3. To display all employees in the company, type - Display all");
    //     println!("4. to exit, type - Exit");

    //     let mut user_input = String::new();

    //     io::stdin()
    //         .read_line(&mut user_input)
    //         .expect("Failed to read line");

    //     user_input = String::from(user_input.trim_end());

    //     if user_input.len() > 0 {
    //         let user_input_parts: Vec<&str> = user_input.split(' ').collect();
    //         if user_input_parts[0] == "Exit" {
    //             break;
    //         }
    //         else {
    //             if user_input_parts[0] == "Add" {
    //                 add_employee_to_dept(user_input_parts[1], user_input_parts[3], & mut emps)
    //             }
    //             else if user_input_parts[0] == "Display" && user_input_parts[1] == "all" {
    //                 display_all_employees(&emps);
    //             }
    //             else {
    //                 display_employees_in_a_dept_asc_ordered(user_input_parts[1], &emps);
    //             }
    //         }
    //     }
    //     else {
    //         println!("Empty user input!! Please enter valid user input: ");
    //     }
    // }

    // // exercises/data_structure/stack.rs
    // const N: usize = 5;
    // let mut stack_object: Stack<u8, N> = Stack::new();

    // stack_object.display(); // empty stack

    // //  push operations
    // stack_object.push(10);
    // stack_object.push(11);
    // stack_object.push(12);

    // stack_object.display(); // partial stack

    // stack_object.push(13);
    // stack_object.push(14);
    // stack_object.push(15);

    // stack_object.display(); // full stack

    // //  pop operations
    // stack_object.pop();
    // stack_object.pop();
    // stack_object.pop();

    // stack_object.display(); // partial stack

    // stack_object.pop();
    // stack_object.pop();
    // stack_object.pop();

    // stack_object.display(); // empty stack

    // exercises/data_structure/queue.rs
    const N: usize = 5;
    let mut queue_object: Queue<u8, N> = Queue::new();

    queue_object.display(); // empty stack

    //  enqueue operations
    queue_object.enqueue(10);
    queue_object.enqueue(11);
    queue_object.enqueue(12);

    queue_object.display(); // partial stack

    queue_object.enqueue(13);
    queue_object.enqueue(14);
    queue_object.enqueue(15);

    queue_object.display(); // full stack

    //  dequeue operations
    queue_object.dequeue();
    queue_object.dequeue();
    queue_object.dequeue();

    queue_object.display(); // partial stack

    queue_object.dequeue();
    queue_object.dequeue();
    queue_object.dequeue();

    queue_object.display(); // empty stack
}
