use std::{
    collections::HashMap,
    // io::{stdin, stdout},
};

type Company = HashMap<String, Vec<String>>;

fn main() {
    let mut company: Company = HashMap::new();

    add_employee(&mut company, "ram".to_string(), "Sales".to_string());
    add_employee(&mut company, "sam".to_string(), "Sales".to_string());
    add_employee(&mut company, "nam".to_string(), "Sales".to_string());
    add_employee(&mut company, "bam".to_string(), "Sales".to_string());

    add_employee(&mut company, "rohan".to_string(), "Engineering".to_string());
    add_employee(&mut company, "mohan".to_string(), "Engineering".to_string());
    add_employee(&mut company, "sohan".to_string(), "Engineering".to_string());
    add_employee(&mut company, "pohan".to_string(), "Engineering".to_string());

    list_department(&company, "Engineering".to_string());

    list_all(&company);
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
            let mut sorted_dept = dept.clone();
            sorted_dept.sort();
            for employee in sorted_dept {
                println!("{}", employee)
            }
        }
        None => println!("department {} not exist", department),
    }
}

fn list_all(company: &Company) {
    for (department, employees) in company {
        print!("{}", department.to_uppercase());
        println!(": {}", employees.len());
        let mut sorted_employees = employees.clone();
        sorted_employees.sort();
        for employee in sorted_employees {
            println!("{}", employee);
        }
    }
}
