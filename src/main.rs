use std::{fs::*, io::Read};
use std::path::{Path};
use std::ffi::OsStr;
use std::env;
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
        // if word == " pwint" {
        //     words[i] = String::from(" print");
        // } 
        if word.contains("ewse"){
            words[i] = String::from(word.replace("ewse", "else"));
        }
    }

    for word in words{
        print!("{}", word);
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
    
    Ok(())
}
 