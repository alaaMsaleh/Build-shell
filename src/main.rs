
// #[allow(unused_imports)]
// use std::io::{self, Write};
// use std::os::unix::fs::PermissionsExt;

// fn main() {
//     // TODO: Uncomment the code below to pass the first stage
//     //to take input

//     let mut input = String::new();

//     loop {
//         //use Reciving user input

//         print!("$ ");
//         io::stdout().flush().unwrap();

//         io::stdin().read_line(&mut input).unwrap();

//         let command = input.trim().to_string();
//         if command.is_empty() {
//             input.clear();
//             continue;
//         }

//         match command.as_str() {
//             "exit" => break,
//             //anonumes condetion
//             text if text.starts_with("echo") => {
//                 println!("{}", &command[5..])
//             }
//             text if text.starts_with("type") => determin_type(&command[5..]),
           
//             _ => println!("{}: command not found", command),
//         }
//         input.clear()
//     }
// }

// fn determin_type(x: &str) {
//     // ireplace if with match bec natch suport OR
//     match x {
//         "echo" | "exit" | "type" => println!("{x} is a shell builtin"),

//         _ => match determin_path(x) {
//                Some(path) => println!("{} is {}", x, path),
//                 None => println!("{}: not found", x),
//         },
//     }
// }

// fn determin_path(comm: &str) -> Option<String> {
//     if let Some(path_env) = std::env::var_os("PATH") {
//         let  parts = std::env::split_paths(&path_env);

//         for z in  parts {
          
//             let full_path = z.join(comm);  //after make path after add input command with pathes takes from os
//             if full_path.is_file() { 
//                // 1. هاتي مواصفات الملف ومكانه
//             if let Ok(metadata) = full_path.metadata() { 
//               let permissions = metadata.permissions();
   
//             if permissions.mode() & 0o111 != 0 {
               
//             return Some(full_path.to_string_lossy().to_string());

//              }
// }
//             }
           

//         }
//     }
//     None
// }

// fn Run_program(path : &str , arrgument :&str )
// {  
//      let child = std::process::Command::new(path)
//      .arg(arrgument).spawn();

//     match child{
//         Ok(child) => {
//             println!("Program started!");
//             println!("PID = {:?}", child.id());
//         },
//         Err(err) => {
//             eprintln!("Error: {}", err);
//         }

  
// }
// }


use std::{env, path};
use std::f32::consts::E;
use std::io::{self, Write};
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use std::os::unix::process::{self, CommandExt};
use std::process::Command;

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        // 1. قراءة وتقسيم الـ Input إلى Command و Arguments
        let (command, args) = parse_input();

        if command.is_empty() {
            continue;
        }

  
        match command.as_str() {
            "exit" => break,

            "echo" => {
                println!("{}", args.join(" "));
            }

            "type" => {
                if let Some(target) = args.first() {
                    determin_type(target);
                }
            }
            "pwd" =>match findcurrent_work_directory(){

                Ok(path) =>println!("{}",path.display()),
                Err(_) => eprintln!("pwd: error retrieving current directory"),
            }

            "cd"=>{
                //take path by remove cd just take pth use input
                if let Some(path_str) = args.first() {
                    //command is string change to path , bec func paramter take path
                    let path_buf = PathBuf::from(path_str);
                    
                    if let Err(_) = change_directory(path_buf) {
                        eprintln!("cd: {}: No such file or directory", path_str);
                    }
                } else {
                    println!("Please provide a directory path.");
                }
            }
         

            // لو مش Built-in، بنبحث عنه في الـ PATH ونشغله
            _ => match determin_path(&command) {
                Some(path) => executing_process(&path, &command,&args),
                None => println!("{}: command not found", command),
            },
        }
    }
}

// دالة قراءة الـ Input وتقسيمه لـ Command و Args كـ Tuple
fn parse_input() -> (String, Vec<String>) {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let mut split = input.split_whitespace();

    let command = split.next().unwrap_or("").to_string();
    let args: Vec<String> = split.map(|s| s.to_string()).collect();

    (command, args)
}

// دالة فحص الـ Built-in أو البحث في الـ PATH لـ type
fn determin_type(x: &str) {
    match x {
        "echo" | "exit" | "type" | "pwd" => println!("{x} is a shell builtin"),

        _ => match determin_path(x) {
            Some(path) => println!("{} is {}", x, path.display()),
            None => println!("{}: not found", x),
        },
    }
}


fn determin_path(comm: &str) -> Option<PathBuf> {
    if let Some(path_env) = env::var_os("PATH") {
        let parts = env::split_paths(&path_env);

        for z in parts {
            let full_path = z.join(comm);
            if full_path.is_file() {
                if let Ok(metadata) = full_path.metadata() {
                    let permissions = metadata.permissions();
                    if permissions.mode() & 0o111 != 0 {
                        return Some(full_path);
                    }
                }
            }
        }
    }
    None
}

fn executing_process(path: &PathBuf,command :&str, args: &[String]) {

   let mut cmd = Command::new(path);
    
    // نحدد أن arg0 هو اسم الأمر فقط (مثل custom_exe_9822) وليس المسار الكامل
    cmd.arg0(command);
    cmd.args(args);

    if let Err(e) = cmd.status() {
        eprintln!("Failed to execute process: {}", e);
    }
}
 
fn findcurrent_work_directory()-> Result<PathBuf, std::io::Error>{
    //Shell(Process) -> Kernal -> Current Work Directory -> return Path

           env::current_dir()

    }

    //create func handel cd => change directory
    fn change_directory(path:PathBuf) -> Result<(), std::io::Error>
    {
        //check path is work
      if !path.is_dir(){
        return Err(io::Error::new(io::ErrorKind::NotFound, "Not a valid directory"));
    }
    env::set_current_dir(path)?;

   
    Ok(())
    }
