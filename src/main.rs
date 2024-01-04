// use std::io::stdin;

//structures have camelcase names
#[derive(Clone, Debug)]
struct User {
    id: i64,
    username: String,
    skills: [String;3],
    stack: Stack

}

#[derive(Clone, Debug)]
enum Stack {
    JS,
    Rust,
    Go,
    Python,
    Hacker
}
fn main() {
    // let mut message:String = String::new();
    // println!("enter a message");

    // stdin().read_line(&mut message).unwrap();
    // println!("received message is: {}", message);

    let my_snake_case_variable:String = String::from("All variables use snake case");
    println!("{}", my_snake_case_variable);


    let my_string:String   = String::from("Snake Case variable here");
    println!("{}", my_string);
    // integers
    let my_integer: i32 =  69;
    println!("{}", my_integer);
    //parsing integers
    let parsed_integer: i32 = "80085".parse().unwrap();
    println!("Parsed integer is {}", parsed_integer);
    println!("integer to string coversion : {}", parsed_integer.to_string());

    // floating point
    let my_float: f32 = 80.085;
    println!("My float value : {}", my_float);
    println!("Float floor : {}", my_float.floor());
    println!("Float ceiling : {}", my_float.ceil());
    println!("Float rounded : {}", my_float.round());

    // working with floats and integers (type conversion)
    let num1: i32 = 10;
    let num2: f32 = 11.5;
    // normal addition isn't possible as we cannot add integers to floating values in rust
    println!("converting int to float, sum is : {}", num1 as f32+num2);
    println!("converting float to int, sum is : {}", num1+num2 as i32);
    // while converting float to int, we lose the precision offered by float. we need to be careful while converting float to int.
    
    //characters
    let my_character: char = 'K';
    println!("Is uppercase : {}", my_character.is_uppercase());
    println!("Is lowercase : {}", my_character.is_lowercase());
    println!("convert to lower case : {}" ,my_character.to_ascii_lowercase());
    // observation : using .to_lowercase() method modifies the value stored in the variable. .to_ascii_lowercase() doesn't modify the stored value
    println!("convert to string : {}", my_character.to_string());

    //booleans
    let my_boolean: bool = false;
    assert_eq!(my_boolean, false);

    //tuples
    let my_tuple: (char,i32,f32,String) = ('K', 69, 80.085, "sulchik".to_string());
    println!("char/int/float/string : {}/{}/{}/{}" , my_tuple.0, my_tuple.1, my_tuple.2, my_tuple.3);
    
    //destructuring tuples
    let (letter, integer, float, string) = my_tuple;
    println!("char/int/float/string : {}/{}/{}/{}" , letter, integer, float, string);
    
    //arrays
    let my_array: [i32;5] = [8,0,0,8,5];

    for num in my_array {
        println!("number : {}", num);
    }

    let same_value_array: [i32;100] = [69;100];
    println!("Array : {:?}", same_value_array);
    println!("first element  :{}", same_value_array[0]);
    println!("array length : {}" ,same_value_array.len());
    println!("array size : {}", std::mem::size_of_val(&same_value_array));

    //structs and enums

    let dino: User = User {
        id:80085,
        username:String::from("dinokage"),
        skills: ["React".to_string(), "Next".to_string(), "Node".to_string()],
        stack: Stack::JS        
    };

    println!("username : {}", dino.username);
    println!("skills : {:?}", dino.skills);

    //combining structs with .. operator

let tarun: User = User {
    id: 80086,
    username: String::from("scoobygamur"),
    ..dino
};

println!("tarun : {:?}", tarun);

// moves and copies

let a:i32 = 69;
let b: i32 = a;
println!("{}",a);
println!("{}",b);

// primitive values are implicitly copied

let string_1: String = String::from("Sulchik");
let string_2: String = string_1;

//println!("string 1 is now gone lmao{}", string_1);
println!("string 2 exists : {}", string_2);

// strings are implicitly moved

// for copying, we use .clone() method (also referred to as deep copy)

let string_3  = string_2.clone();
println!("both string_2 and string_3 exist now : {} {}", string_2, string_3);

// values stored in stack are copied by default. values stored in heap are moved by default
// as strings are of variable size they are stored in heap
{
    let random_string: String = String::from("Sulli");
}
// println!("{}", random_string);
// a variable is fdropped once ir exits it's scope

let karna: User = User { id: 80087, username: String::from("cpt.karna"), skills: ["Ethukelli Hacking".to_string(), "Mind Reading".to_string(), "Hadavidi".to_string()], stack:  Stack::Hacker};
// for some reason .clone() method didn't work with my struct
//  it worked :) [had to include "Clone" in derive attribute for struct and enum]
let puchakai: User = karna.clone();
println!("{:?} {:?}" ,karna ,puchakai);
}
