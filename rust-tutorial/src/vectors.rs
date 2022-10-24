// vector -resizable arrays
pub fn run(){
    let mut numbers:Vec<i32>=vec![1,2,3,4,5];
    println!("{:?}",numbers);
    //Get single value 
    println!("first number {}",numbers[0]);
    //re-assign value
    numbers[2]=20;
    println!("{:?}",numbers);
    // vector are stack allocated 
    println!("vector occupies {} bytes",std::mem::size_of_val(&numbers));
    //Get slice first two elements
    let slice:&[i32]=&numbers[0..2];
    println!("slice: {:?}",slice);
    //add on to vector
    numbers.push(6);
    println!("{:?}",numbers);
    //pop off last value from
    numbers.pop();
    println!("{:?}",numbers);
    //loop through vector values
    for x in numbers.iter(){
        println!("Number: {}",x);
    }
    //loop and mutate values similar to map
    for x in numbers.iter_mut(){
        *x *=2;
    }
    println!("Number: {:?}",numbers);
}