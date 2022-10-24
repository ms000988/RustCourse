pub fn run(){
    //print to console
    println!("Hello from println function");
    println!("Number:{}",1);
    //Basic Formatting
    println!("{} is from {}","Brad","Mass");
    //possitional Arguments
    println!("{0} is from {1} and {0} likes to {2}","Brad","Mass","code");
    //Named Arguments
    println!("{name} likes to play {activity}",name="Mustafa",activity="Baseball");
    //placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}",10,10,10);
    // Placeholder for debug trait tuple
    println!("{:?}", (12,true,"hello"));
    //basic math
    println!("10 + 101 = {}",10+10 );
}