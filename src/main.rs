use std::collections::HashMap;
use std::io::{self, Write};

// Struct to hold one set of login credentials
struct Credential {
    username: String,
    password: String,
}

fn main() {
    // This HashMap stores all credentials, using the site name as the key
    let mut credentials: HashMap<String, Credential> = HashMap::new();

    loop {
        print_menu();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice = choice.trim();

        match choice {
            "1" => add_credential(&mut credentials),
            "2" => list_credentials(&credentials),
            "3" => delete_credential(&mut credentials),
            "4" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid choice. Try again."),
        }
    }
}

// Displays the main menu options
fn print_menu() {
    println!("\nWelcome to the Rust Credential Manager!");
    println!("1. Add a new credential");
    println!("2. List all credentials");
    println!("3. Delete a credential");
    println!("4. Quit");
    print!("Enter your choice: ");
    io::stdout().flush().unwrap(); // Flush ensures prompt appears immediately
}

// Prompts user for site, username, and password and adds to HashMap
fn add_credential(credentials: &mut HashMap<String, Credential>) {
    let mut site = String::new();
    let mut username = String::new();
    let mut password = String::new();

    println!("\n--- Add a New Credential ---");

    print!("Enter site name: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut site).unwrap();

    print!("Enter username: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut username).unwrap();

    print!("Enter password: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut password).unwrap();

    // Trim and convert all input to owned Strings
    let site = site.trim().to_string();
    let username = username.trim().to_string();
    let password = password.trim().to_string();

    let credential = Credential { username, password };
    credentials.insert(site.clone(), credential);

    println!("Credential for '{}' added.", site);
}

// Loops through and prints all stored credentials
fn list_credentials(credentials: &HashMap<String, Credential>) {
    if credentials.is_empty() {
        println!("\nNo credentials stored.");
        return;
    }

    println!("\n--- Stored Credentials ---");
    for (site, cred) in credentials.iter() {
        println!("Site: {}", site);
        println!("  Username: {}", cred.username);
        println!("  Password: {}", cred.password);
    }
}

// Removes a credential from the map based on site name
fn delete_credential(credentials: &mut HashMap<String, Credential>) {
    let mut site = String::new();

    println!("\n--- Delete a Credential ---");
    print!("Enter the site name to delete: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut site).unwrap();

    let site = site.trim();

    if credentials.contains_key(site) {
        credentials.remove(site);
        println!("Credential for '{}' deleted.", site);
    } else {
        println!("No credential found for '{}'.", site);
    }
}
