#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    // TODO: Uncomment the code below to pass the first stage
    //to take input
  
   let built_in="exit";
   let mut input= String::new();
     loop{
    //use Reciving user input
 

    
   
    print!("$ ");
    io::stdout().flush().unwrap();


  
    io::stdin().read_line(&mut input).unwrap();
    if input.trim()==built_in
     {
        break;
    }
    println!("{}: command not found",input);

    input.clear()
   
}
}
