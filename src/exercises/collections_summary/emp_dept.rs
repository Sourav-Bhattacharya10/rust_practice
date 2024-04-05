use std::collections::HashMap;

#[derive(Debug)]
pub struct Employee {
    pub emp_id: u16,
    pub name: String,
    pub dept: String,
    pub dept_id: u16
}



pub fn add_employee_to_dept(name: &str, dept: &str, emps: & mut Vec<Employee>){
    let mut dept_hash_map: HashMap<String, u16> = HashMap::new();
    dept_hash_map.insert(String::from("Engg"), 1);
    dept_hash_map.insert(String::from("Sales"), 2);
    dept_hash_map.insert(String::from("Marketing"), 3);

    let dept_id: u16 =  *dept_hash_map.get(dept).unwrap();
    emps.push(Employee {emp_id: emps.len() as u16 + 1, name: String::from(name), dept_id: dept_id, dept: String::from(dept)})
}

pub fn display_employees_in_a_dept_asc_ordered(dept: &str, emps: &Vec<Employee>){
    if emps.len() == 0 {
        println!("No employees in the company. Please add few employees first!!");
    }
    else{
        println!("Employees in Department - {} are :", dept);
        let dept_emps = emps.iter().filter(|emp| emp.dept == dept);
        for dept_emp in dept_emps {
            println!("Employee : {:?}", dept_emp);
        }
    }
}

pub fn display_all_employees(emps: &Vec<Employee>){
    if emps.len() == 0 {
        println!("No employees in the company. Please add few employees first!!");
    }
    else{
        for emp in emps {
            println!("Employee : {:?}", emp);
        }
    }
}