use std::fs::OpenOptions;
use std::io::Write;
use std::io;

struct Data{
    website : String,
    email: String,
    password: String,
}
//Read data function

fn read_data() {
    let content = std::fs::read_to_string("password.txt")
        .expect("Failed to read file");
    println!("{}", content);
}

//Add data function

fn add_data() {
    //getting data from website
    
    let mut website = String::new();
    println!("\nEnter your website URL: ");
    io::stdin()
        .read_line(&mut website)
        .expect("Failed to read line");
    let website = website.trim(); // Remove any trailing newline characters
    //getting data from email
    
    let mut email = String::new();
    println!("\nEnter your email address: ");
    io::stdin()
        .read_line(&mut email)
        .expect("Failed to read line"); 
    let email = email.trim(); // Remove any trailing newline characters
    //Now the password

    let mut password = String::new();
    println!("\nEnter your password: ");
    io::stdin()
        .read_line(&mut password)
        .expect("Failed to read line");
    let password = password.trim(); // Remove any trailing newline characters

    let data = Data {
        website: website.to_string(),
        email: email.to_string(),
        password: password.to_string(),
    };
    println!("\n Data collected successfully!");
    let content = format!(
        "Website: {}\nEmail: {}\nPassword: {}\n",
        data.website,
        data.email,
        data.password
    );
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("password.txt")
        .expect("Failed to open file");
    file.write_all(content.as_bytes())
        .expect("Failed to write to file");
}
fn main() {
    println!("Welcome to the Password Manager!");
    loop {
        let mut option = String::new();
        println!("\nPlease select an option:");
        println!("1. Read data");
        println!("2. Add data");
        println!("3. Exit");
        io::stdin()
            .read_line(&mut option)
            .expect("Failed to read line");
        let option = option.trim(); // Remove any trailing newline characters
        if option == "1"{
            read_data();
        }
        else if option == "2"{
            add_data();
        }
        else if option == "3"{
            println!("Exiting the program...");
            break;
        }
        else{
            println!("Invalid option. Please try again.");
        }
    }

}