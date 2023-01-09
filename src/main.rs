use std::io;

fn main() {

    println!("------------Sign Up------------");
    let mut username = String::new();
    println!("Enter your username: ");
    io::stdin().read_line(&mut username).expect("Error");

    let mut password = String::new();
    println!("Enter your password: ");
    io::stdin().read_line(&mut password).expect("Error");

    {
    let mut check_password = String::new();
    println!("Confirm your password: ");
    io::stdin().read_line(&mut check_password).expect("Error");

    if check_password != password {
    println!("Password does not match.");
    return;
    }
    }

    println!("\n----------Login----------");
    {
    let mut check_username = String::new();
    println!("Enter your username: ");
    io::stdin().read_line(&mut check_username).expect("Error");
    if check_username != username{
        println!("Username does not match.");
        return;
    } 
    }
    {
    let mut login_password = String::new();
    println!("Enter your password: ");
    io::stdin().read_line(&mut login_password).expect("Error");
     if login_password != password{
        println!("Password does not match.");
        return;
    }
    }
    
    println!("\nAccessing system...");
}
