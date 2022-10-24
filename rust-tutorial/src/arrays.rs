// Arrays - Fixed list where elements are the same data type
pub fn run(){
    let mut numbers:[i32;5]=[1,2,3,4,5];
    println!("{:?}",numbers);
    //Get single value 
    println!("first number {}",numbers[0]);
    //re-assiign value
    numbers[2]=20;
    println!("{:?}",numbers);
    // Arrays are stack allocated 
    println!("Array occupies {} bytes",std::mem::size_of_val(&numbers));
    //Get slice first two elements
    let slice:&[i32]=&numbers[0..2];
    println!("slice: {:?}",slice);
}