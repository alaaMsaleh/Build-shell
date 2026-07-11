#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    // TODO: Uncomment the code below to pass the first stage
    //to take input
  
   
   let mut input= String::new();
     loop{
    //use Reciving user input
 
   
    print!("$ ");
    io::stdout().flush().unwrap();


  
    io::stdin().read_line(&mut input).unwrap();

    let command = input.trim().to_string();
    if command=="exit"
     {
        break;
    }
    else if command.starts_with("echo"){
        
        println!("{}",&command[5..]);
    }else{
    println!("{}: command not found",command);
    }
    input.clear()

   
}
}
