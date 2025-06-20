use std::{
    collections::HashMap,
    // io::{stdin, stdout},
};

type Company = HashMap<String, Vec<String>>;

fn main() {
    let mut company: Company = HashMap::new();

    add_employee(&mut company, "rohan".to_string(), "Engineering".to_string());
    add_employee(&mut company, "mohan".to_string(), "Engineering".to_string());
    add_employee(&mut company, "ohan".to_string(), "Engineering".to_string());
    add_employee(&mut company, "sohan".to_string(), "Engineering".to_string());
    list_department(&company, "Engineerin".to_string());
}

fn add_employee(company: &mut Company, employee: String, department: String) {
    let dept = company
        .entry(department.to_lowercase())
        .or_insert(Vec::new());

    dept.push(employee.to_lowercase());
}

fn list_department(company: &Company, department: String) {
    match company.get(&department.to_lowercase()) {
        Some(dept) => {
            for employee in dept {
                println!("{}", employee)
            }
        }
        None => println!("department {} not exist", department),
    }
}
