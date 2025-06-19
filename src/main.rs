use std::{
    collections::HashMap,
    // io::{stdin, stdout},
};

type Company = HashMap<String, Vec<String>>;

fn main() {
    let mut company: Company = HashMap::new();

    add_employee(&mut company, "Rohan".to_string(), "Engineering".to_string());
}

fn add_employee(company: &mut Company, employee: String, department: String) {
    let dept = company
        .entry(department.to_lowercase())
        .or_insert(Vec::new());

    dept.push(employee.to_lowercase());
}
