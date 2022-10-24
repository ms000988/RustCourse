//primitive str = Immutable fixed-length string somewhere in memory.\
//String = Growable, heap-allocated data structure - Use when you need to modify or own string
//data
pub fn run(){
    let mut hello=String::from("Hello ");

    //Get length
    hello.push('W'); //push a char not a string 
    hello.push_str("orld");
    println!("{} is lenght {}", hello,hello.len());
    //capacity in bytes
    println!("{} capacity", hello.capacity());
    //is empty
    println!("{} is empty", hello.is_empty());
    //Contains
    println!("{} contains 'World'",hello.contains("World"));
    //replace
    println!("replace: {}",hello.replace("World","There"));
    //loop through string by whitespace
    for word in hello.split_whitespace(){
        println!("{}",word);
    }
    //Create string with capacity
    let mut s=String::with_capacity(10);
    s.push('a');
    s.push('b');
    println!("{}",s);
    //assertion testing
    assert_eq!(2,s.len());

}