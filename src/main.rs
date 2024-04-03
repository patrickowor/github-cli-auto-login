#![allow(non_snake_case)]

use std::env;
use std::process::Command;
use std::str;
use std::fs;



fn command(cmd_arg : &str ) -> String{
    let output = if cfg!(target_os = "windows") {
        Command::new("powershell")
            .args(["/C", cmd_arg])
            .output()
            .expect("failed to execute process")
    } else {
        Command::new("sh")
            .arg("-c")
            .arg("echo hello")
            .output()
            .expect("failed to execute process")
    };

    str::from_utf8(&output.stdout)
            .expect("cant parse standard output into byte")
            .replace("\r\n", "")
}

fn format<'a> (val : String) -> String{
    format!("echo {} > token.txt ; Get-Content token.txt | gh auth login --with-token", val )
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Error : no argument passed");
        return ();
    }
    let _ =command("gh auth logout");
    match args[1].as_str() {
        "token" => {
            if args.len() < 3 {
                println!("Error : no Token passed!");
                return ();
            }
            let result = command(format(args[2].clone()).as_str());
            
            if result == String::from("") {
                let _ =command("gh auth setup-git");
                let r =command("gh auth status");
                println!("{r}");
            }
        }
        _ => {
            let file_data =fs::read_to_string("git.txt")
        .expect("Error: Unable to read the file git.txt");
            let GIT_TOKENS: &str = file_data.as_str();
            //include_str!("git.txt");
            let mut loop_ended = true;
            for token_and_name in GIT_TOKENS.split("\r\n").collect::<Vec<&str>>() {
                let parts = token_and_name.split(" ").collect::<Vec<&str>>();
                if parts[0] == args[1] {
                    // println!("{}, {}",args[1], parts[0]);
                    let result = command(format(parts[1].to_string()).as_str());
                    if result == String::from("") {
                        let _ =command("gh auth setup-git");
                        let r =command("gh auth status");
                        println!("{r}");
                    }
                    loop_ended = false;
                    break;
                }
            }
            if loop_ended { println!("Error : key '{}' not found!", args[1])}
        }
    }
}
