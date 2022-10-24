pub fn run(){
    let mut count=0;
    //infinit loop
    // loop{
    //     count += 1;
    //     println!("{}",count);
    //     if count == 20{
    //         break;
    //     }
    // }
    //while loop (FizzBuzz)
    // while count <=100{
    //     if count%15==0{
    //         println!("fizzbuzz");
    //     }
    //     if count%3==0{
    //         println!("fizz");
    //     }
    //     if count%5==0{
    //         println!("buzz")
    //     }
    //     else{
    //         println!("{}",count);
    //     }
    //     count += 1;
    // }
    //for range 
    for mut x in 0..100{
        if x%15==0{
            println!("fizzbuzz");
        }
        if x%3==0{
            println!("fizz");
        }
        if x%5==0{
            println!("buzz")
        }
        else{
            println!("{}",x);
        }
        x += 1;
    }
}