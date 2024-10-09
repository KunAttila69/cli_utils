use std::{fs, io}; 
fn main() {
    
    loop {
        let mut command= String::new();
        println!("Please input your command.");
        io::stdin()
            .read_line(&mut command) 
            .expect("Failed to read line");
        parse_command(&command); 
    }
}

fn parse_command(command: &str) {
    let chunks: Vec<&str> = command.split_whitespace().collect();
    if chunks.is_empty() {
        return; 
    }
    
    match chunks[0] {
        "echo" => {
            if chunks.len() > 1 { 
                let args = &chunks[1..].join(" ");
                echo(args);
            } else {
                println!("No arguments provided for echo.");
            }
        },
        "ls" => {
            if chunks.len() == 2 {
                ls(chunks[1]);                
            }else if chunks.len() == 1{
                ls("./files");
            }else{
                println!("Invalid parameters given")
            }
        },
        "grep" => {
            if chunks.len() == 3{
                grep(chunks[2], chunks[1]);                
            }else if chunks.len() == 2{
                grep("./files", chunks[1]);
            }else{
                println!("Invalid parameters given")
            }
        }
        _ => println!("Unknown command: {}", chunks[0]),
    }
}


fn echo(param: &str) {
    println!("{param}");
}

// fn cat(){

// }

fn ls(path: &str){
    let paths = match fs::read_dir(path) {
        Ok(paths) => paths,
        Err(_) => {
            println!("Invalid path given.");
            return;
        } 
    };
 

    for path in paths {
        println!("{}", path.unwrap().path().display())
    }
}

// fn find(){

// }

fn grep(file_path: &str, text: &str){
    let paths = match fs::read_dir(file_path) {
        Ok(paths) => paths,
        Err(_) => {
            println!("Invalid path given.");
            return;
        } 
    };
 
    let mut txt_files: Vec<String> = Vec::new();
    for path in paths {
        let entry = match path {
            Ok(entry) => entry,
            Err(_) => continue,
        };

        let path = entry.path();
        if let Some(path_str) = path.to_str() {
            if path_str.ends_with("txt") {
                txt_files.push(path.display().to_string());
            }
        }
    }
}