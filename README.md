# 🏢 company-director-rs

> A simple interactive Rust CLI app to manage company employees and departments 🦀

---

## ✨ Features

- ➕ Add employees to departments via natural language commands
- 📋 List all employees in any department (alphabetically sorted)
- 🏢 List all departments and their employees
- 🔒 Safe and efficient using Rust's HashMap and Vector collections
- ⚡ Fully interactive text interface

---

## 🚀 Example Commands

```bash
add Sally to Engineering
add Amir to Sales
list Engineering
list Sales
list all
exit
```

Example output:

```
Department: Engineering
- Sally

Department: Sales
- Amir
```

---

## 🧑‍💻 Why Rust?

* 🦀 Memory safety without garbage collection
* ⚡ High performance and zero-cost abstractions
* 📚 Excellent for learning ownership, collections, and string parsing

---

## 🔧 Installation

First, make sure you have Rust installed:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Then clone the repository:

```bash
git clone git@github.com:SadiqOnGithub/rust-company_director.git
cd rust-company_director
```

Build and run:

```bash
cargo run
```

---

## 🤝 Contributions

Contributions, issues and pull requests are welcome!

---

## 📄 License

This project is licensed under the MIT License.
