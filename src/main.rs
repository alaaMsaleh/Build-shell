
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


use std::env;
use std::io::{self, Error, ErrorKind, Write};
use std::os::unix::fs::PermissionsExt;
use std::os::unix::process::CommandExt;
use std::path::{Path, PathBuf};
use std::process::Command;
fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        //Row String
        let single_qoute =r#"''"#;
        let double_qoute =r#"''"#;

        /*how terminal work with Qouting
 
        1- frist take Statment input and detemin type of qouting
        2- then remove and execute command 



         */
       

        // frist step at shell
        let (command, args) = parse_input_qouting();

        if command.is_empty() 
        {
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
                let path_str = args.first().map(|s| s.as_str()).unwrap_or("~");
                let path_buf = PathBuf::from(path_str);
            
                if let Err(_) = change_directory(path_buf) {
                    eprintln!("cd: {}: No such file or directory", path_str);
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

// fun to handel input data
fn parse_input_qouting() -> (String, Vec<String>) {

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let mut args = Vec::new(); //ر
    let mut current_arg = String::new();

    //remove \n or \r\n in input
    let trimmed_input = input.trim_end();

    let mut in_single_quote = false;

    for ch in  trimmed_input.chars(){
        match ch {
            '\'' => {
                in_single_quote = !in_single_quote;
            }

            ' ' if !in_single_quote => {
                if !current_arg.is_empty() {
                    args.push(current_arg.clone());
                    current_arg.clear();
                }
         }
         _=>{
            current_arg.push(ch);
         }

        }
    }

    if !current_arg.is_empty() {
        args.push(current_arg);
    }

    if args.is_empty() {
        return (String::new(), Vec::new());
    }
    
    let command = args.remove(0); // أول كلمة هي الـ Command
    (command, args)
}
//input ech "Hello World" =>(Where arguments start and end) , Instructions to Parser



// دالة فحص الـ Built-in أو البحث في الـ PATH لـ type
fn determin_type(x: &str) {
    match x {
        "echo" | "exit" | "type" | "pwd" |"cd" => println!("{x} is a shell builtin"),

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
    pub fn change_directory(path: PathBuf) -> Result<(), io::Error> {
        // 1. Resolve the path (Expand ~ if needed)
        let expanded_path = expand_tilde(&path)?;
    
        // 2. Delegate the actual directory change and validation to the OS via std::env
        // set_current_dir automatically checks existence, directory status, and permissions.
        env::set_current_dir(expanded_path)
    }
    
    /// Helper function responsible strictly for tilde expansion.
    /// Keeps separation of concerns clean.
    fn expand_tilde(path: &Path) -> Result<PathBuf, io::Error> {
        // 1. التشييك لو المسار هو "~" أو يبدأ بـ "~/"
        if path == Path::new("~") || path.starts_with("~/") {
            let home_dir = env::var("HOME").map_err(|_| {
                Error::new(
                    ErrorKind::NotFound,
                    "HOME environment variable is not set",
                )
            })?;
    
            // لو المسار هو "~" فقط
            if path == Path::new("~") {
                return Ok(PathBuf::from(home_dir));
            }
    
            // لو المسار مثلاً "~/desktop"
            if let Ok(relative_path) = path.strip_prefix("~/") {
                return Ok(PathBuf::from(home_dir).join(relative_path));
            }
        }
    
        // المسارات العادية
        Ok(path.to_path_buf())
    }