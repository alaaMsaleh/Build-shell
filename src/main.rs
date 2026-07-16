
#[allow(unused_imports)]
use std::io::{self, Write};
use std::{fs::{metadata, Permissions}, os::unix::fs::PermissionsExt};

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
        if command.is_empty() {
            input.clear();
            continue;
        }

        match command.as_str() {
            "exit" => break,
            //anonumes condetion
            text if text.starts_with("echo") => {
                println!("{}", &command[5..])
            }
            text if text.starts_with("type") => determin_type(&command[5..]),
            _=>{
            let parts: Vec<String> = command.split_whitespace().map(|s| s.to_string()).collect();
                
                // أول كلمة هي اسم البرنامج (مثلاً my_exe)
                let program_name = &parts[0];
                
                // باقي الكلمات هي الـ arguments (من العنصر رقم 1 لآخر الـ vector)
                let arguments = &parts[1..];

                // 2. البحث عن البرنامج في الـ PATH
                match determin_path(program_name) {
                    Some(full_path) => {
                        // 3. تشغيل البرنامج وتمرير الـ arguments له إذا وجدناه
                        Run(&full_path, arguments);
                    }
                    None => {
                        // 4. إذا لم نجده، نطبع الرسالة الشهيرة
                        println!("{}: command not found", program_name);
        }}
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
          
            let full_path = z.join(comm);  //after make path after add input command with pathes takes from os
            if full_path.is_file() { 
               // 1. هاتي مواصفات الملف ومكانه
            if let Ok(metadata) = full_path.metadata() { 
              let permissions = metadata.permissions();
   
            if permissions.mode() & 0o111 != 0 {
               
            return Some(full_path.to_string_lossy().to_string());

             }
}
            }
           

        }
    }
    None
}
pub  fn Run(program_path : & str, argumment  : &[String]){

    let  mut child = std::process::Command::new(program_path)
    .args(argumment)
    .spawn();

    match child {
        Ok(mut child_process) => {
            // 4. السطر ده مهم جداً: بنقول للـ Shell استني لغاية ما البرنامج الخارجي يخلص
            let _ = child_process.wait();
        }
        Err(e) => {
            // لو حصل مشكلة أثناء محاولة التشغيل
            eprintln!("Failed to execute process: {}", e);
        }
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
