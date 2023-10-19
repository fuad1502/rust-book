use std::{collections::HashMap, io};

fn main() {
    let mut department_employees: HashMap<String, Vec<String>> = HashMap::new();
    loop {
        // Get name
        let mut name = String::new();
        println!("Name: ");
        io::stdin().read_line(&mut name).expect("IO Error");
        name = name.trim().to_string();
        // Exit if user entered "exit"
        if name == "exit" {
            break;
        }

        // Get department
        let mut department = String::new();
        println!("Department: ");
        io::stdin().read_line(&mut department).expect("IO Error");
        department = department.trim().to_string();

        // Push to HashMap
        let entry = department_employees.entry(department).or_insert(Vec::new());
        entry.push(name);
    }

    // Print result
    for (department, employees) in &mut department_employees {
        employees.sort();
        println!("Department {department}:");
        println!("{:?}", employees);
    }
}
