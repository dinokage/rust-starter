use std::io::stdin;

fn main() {
    let mut message:String = String::new();
    println!("enter a message");

    stdin().read_line(&mut message).unwrap();
    println!("received message is: {}", message);

    let my_snake_case_variable:String = String::from("All variables use snake case");
    println!("{}", my_snake_case_variable);


    let my_string:String   = String::from("Hello, World");
    println!("{}", my_string);



}
