# rust-cli-vault

## Overview

This project is a simple command-line credential manager written in the Rust programming language. It allows users to securely store, view, and delete login credentials (site or app name, username, password) locally in memory. While the data isn’t encrypted yet, the software was built with extendability in mind and is structured in a way that could support encryption or persistence in the future.

My goal in writing this software was to deepen my understanding of the core syntax and language features of Rust by building a practical tool. This project helped me get hands-on with structs, HashMaps, pattern matching, ownership, borrowing, and user input. It gave me a chance to really wrestle with the Rust compiler and learn how to work with its strict safety and memory model.

[Software Demo Video](DONT FORGET TO ADD ME!!!!!!)


### Development Environment

- Visual Studio Code (with Rust extensions)
- Git for version control
- Cargo package manager
- Windows 11 Pro


This Project was written entirely in **Rust**, using only the standard library and no external crates. I chose to do this intentionally to get a bit more comfortable with the fundamentals of Rust.


### Useful Websites
- [The Official Rust Documentation Website](https://doc.rust-lang.org/stable/)
- [KDnuggets' *A Gentle Introduction to Rust for Python Programmers*](https://www.kdnuggets.com/gentle-introduction-rust-python-programmers)
- [tutorialspoint *Rust for Absolute Beginners*](https://www.tutorialspoint.com/rust/index.htm)


### Future Work

- Add AES encryption for stored credentials
- Save/load from JSON or encrypted file
- Hide character input during password entry
- Add input validation and error handling for edge cases