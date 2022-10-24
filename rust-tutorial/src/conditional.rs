//conditionals - used to check the condition of something and act on the result
pub fn run(){
 let age =20;
 let check_id = false;
 //IF/Else
 if age>=21 && check_id{
     println!("Bartender: What would you like to drink?");
 }
 else if age<=21 && check_id {
     println!("Bartender: Sorry, you have to leave");
 }
 else {
     println!("Bartender:I'll need to see you id ");
 }
 //shorthand if 
 let is_of_age=if age>=21{true} else {false};
 println!("is_of_age {}",is_of_age);
}