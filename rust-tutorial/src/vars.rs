//Variables hold primitive data or references to data 
//Variables are immutable by default 
//Rust is a block-scoped language
pub fn run(){
let name = "Mostafa";
let mut age = 33;
age = 34;
println!("My name is {} and I am {}", name,age );

//Define constant 
const ID:i32=001;
println!("ID: {}",ID);

//assign multiple variables
let (my_name, my_age) = ("Mostafa",34);
println!("My name is {} and I am {}",my_name,my_age);
}