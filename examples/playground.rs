use std::fmt::Arguments;
use std::io::{self, Write};
use std::env::{self, args, var_os};
use std::os::unix::process;
use std::path::{self, PathBuf};
use std::process::{Child, Command};
use std::vec;
fn main()
{

     let (command , args)=parse_input();
     println!("{:?},{:?}",command,args);

     match find_in_path(&command){
        Some(path) => println!("Found program at: {:?}", path),
        None => println!("Program not found!"),
    }
     
 
}

//use to return data type => tuple/*

//what tuple is compound type =container to collect defferent data as one value
fn parse_input () ->(String , Vec<String>) //
{
    let mut input = String::new();
    std::io::stdin()
    .read_line( &mut input) //func readline need mut refernce not variable
    .expect("Faild to read line");

    let mut spilt = input.split_whitespace(); //split is type spiltWithspace is iterator and it in rust not have index so we work with as pointer need method to check element < becouse iteratoe is peoduce element om demand
    
  
   let command = spilt.next().unwrap_or("default").to_string(); //i use to string bec next return &str and i determin data type String
   
   let args: Vec<String> = spilt.map(|s| s.to_string()).collect();
   
   //return tuple
   (command , args)

}

fn find_in_path(command : &str)->Option<PathBuf>{

      
      //1 - frist read "PATH" and Split 
      if let Some (path_os) = env::var_os("PATH"){

        let paths = env::split_paths(&path_os);

          //2- search at all Folders

          for path in paths{
            let full_path = path.join(command);
            if full_path.is_file(){

                return Some(full_path);
            }
          }


      }
   

    None
}


fn executing_process (path : &PathBuf , args : &[String])
{
    
   
   
   //create child process
    match Command::new(path)
   .args(args)
    .status(){

        Ok(status) => println!("Process finished with status: {}", status),
        Err(e) => eprintln!("Failed to execute process: {}", e),
    }

 
}