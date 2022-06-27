use std::{fs::*, io::Read, io::Write};
use std::path::{Path};
use std::ffi::OsStr;
use std::{env, fs};
use colored::Colorize;

fn loadfile(path: &Path, string: &mut String) -> std::io::Result<()> { 
    let extension = path.extension().and_then(OsStr::to_str);

    match extension{
        None => println!("no extension!"),
        Some(extension) => {
            if extension == "pyowo" {
                let file = File::open(path);
                let mut file = match file{
                    Ok(file) => file,
                    Err(_error) => {
                        println!("{}", format!("Error: Couldn't open file: {:?} ", path).red().bold());
                        std::process::exit(0);
                    },
                };

                file.read_to_string(string)?; 
                
                // println!("{}", contents);
            } else{
                println!("{}" , format!("Error: Please provide a .pyowo file!").red().bold());
            }
        }
    }

    Ok(())

}

fn to_words(string : &mut String) -> Vec<String>{
    let bytes = string.as_bytes();

    let mut start = 0;
    let mut words : Vec<String> = Vec::new();
    for (i, &byte) in bytes.iter().enumerate(){
        if byte == b' ' || byte == b'(' || byte == b':' || byte == b'\n' {
            words.push(string[start..i].to_string());
            start = i;
        }
    }

    words.push(string[start..].to_string());

    words
}

fn parse(words: &mut Vec<String>){
    let iter = words.clone();
    for (i, word) in iter.iter().enumerate(){
        if word.contains("pwint"){
            words[i] = String::from(word.replace("pwint", "print"));
        } 
        if word.contains("ewse"){
            words[i] = String::from(word.replace("ewse", "else"));
        }
        if word.contains("ewif"){
            words[i] = String::from(word.replace("ewif", "elif"));
        }
    }

}

fn createfile(path: &Path, string: &String){
    let mut new_path = env::current_dir().unwrap();

    new_path.push("output");
    fs::create_dir_all(&new_path).expect("test");
    new_path.push(path);
    new_path.set_extension("py");
    let path = new_path.clone();
    let newfile = File::create(new_path);

    let mut newfile = match newfile{
        Ok(file) => file,
        Err(_error) => {
            println!("{}", format!("Error: Couldn't create file: {} ", path.display()).red().bold());
            // println!("{}", _error);
        	std::process::exit(0);
        }
    };

    let result = write!(newfile, "{}", string);

    match result{
        Ok(_res) => (),
        Err(_err) => {
            println!("{}", format!("Something went wrong writing the file").red().bold());
            std::process::exit(0)
        }
    }
    
}

fn main() -> std::io::Result<()> {

    let args: Vec<String> = env::args().collect();

    if args.len() < 2{
        eprintln!("{}", format!("Error: Please provide a file to compile").red().bold());
        std::process::exit(0);
    } else if args.len() > 2{
        eprintln!("{}", format!("Error: Too many arguments! expecteed 1, got {}. Please only select one file.", args.len()-1).red().bold());
        std::process::exit(0);
    }

    let path = Path::new(args[1].as_str());
    let mut string = String::new();
    loadfile(path, &mut string)?;
    let mut words = to_words(&mut string);

    parse(&mut words);
    
    string = words.join("");

    createfile(path, &string); 

    let mut filepath = env::current_dir().unwrap();
    filepath.push("output");
    filepath.push(path);
    filepath.set_extension("py");


    let command = std::process::Command::new("python3").arg(filepath).output();

    match command {
        Ok(output) => {

            if !output.stdout.is_empty(){
                println!("{}", String::from_utf8(output.stdout).unwrap());
            }

            if !output.stderr.is_empty(){
                println!("{}", String::from_utf8(output.stderr).unwrap());
            }

        },
        Err(error) => {
            println!("{}", error);
            std::process::exit(0);
        }
    };


    Ok(())
}
 