use std::{
    collections::HashMap,
    // io::{stdin, stdout},
};

type Company = HashMap<String, Vec<String>>;

fn main() {
    let mut company: Company = HashMap::new();
    println!("{:#?}", company);
    // let mut input = String::new();

    add_employee(&mut company, "Rohan".to_string(), "Engineering".to_string());
}

fn add_employee(company: &mut Company, employee: String, department: String) {
    let dept = company
        .entry(department.to_lowercase())
        .or_insert(Vec::new());

    dbg!(&dept);
    dept.push(employee.to_lowercase());

    dbg!(&dept);
    dbg!(&company);
}
