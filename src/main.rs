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

   
    match command.as_str(){

        "exit" =>break,
        //anonumes condetion
       text if text.starts_with("echo")
        =>{ println!("{}",&command[5..])},
        text if text.starts_with("type")=>determin_type(&command[5..]) ,
        _=> println!("{} :not found",command)
    }

    fn determin_type(x : &str){
        
        // ireplace if with match bec natch suport OR
       match x {
        "echo"|"exit"|"type"=>println!("{x} is a shell builtin"),
        _=>println!("{x} Not found"),

       }
    }
    // if command=="exit"
    //  {
    //     break;
    // }
    // else if command.starts_with("echo"){
        
    //     println!("{}",&command[5..]);
    // }else{
    // println!("{}: command not found",command);
    // }
    input.clear()

   
}
}
