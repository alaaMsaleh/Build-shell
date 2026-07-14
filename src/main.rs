
#[allow(unused_imports)]
use std::io::{self, Write};
use std::os::unix::fs::PermissionsExt;

fn main() {
    // TODO: Uncomment the code below to pass the first stage
    //to take input

    let mut input = String::new();

    loop {
        //use Reciving user input

        print!("$ ");
        io::stdout().flush().unwrap();

        io::stdin().read_line(&mut input).unwrap();

        let command = input.trim().to_string();

        match command.as_str() {
            "exit" => break,
            //anonumes condetion
            text if text.starts_with("echo") => {
                println!("{}", &command[5..])
            }
            text if text.starts_with("type") => determin_type(&command[5..]),
            _ => println!("{}: command not found", command),
        }
        input.clear()
    }
}

fn determin_type(x: &str) {
    // ireplace if with match bec natch suport OR
    match x {
        "echo" | "exit" | "type" => println!("{x} is a shell builtin"),

        _ => match determin_path(x) {
            Some(path) => println!("{} is {}", x, path),
            None => println!("{}: not found", x),
        },
    }
}

fn determin_path(comm: &str) -> Option<String> {
    if let Some(path_env) = std::env::var_os("PATH") {
        let  parts = std::env::split_paths(&path_env);

        for z in  parts {
          
            let full_path = z.join(comm);
            if full_path.is_file() {
                if let Ok(metadata) = full_path.metadata() {
                    let permissions = metadata.permissions();
                    // 0o111 للـ Unix تتأكد أن أحد خانات الـ Execute مفعّلة
                    if permissions.mode() & 0o111 != 0 {
                        return Some(full_path.to_string_lossy().to_string());
                    }
                }
            }
           

        }
    }
    None
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
