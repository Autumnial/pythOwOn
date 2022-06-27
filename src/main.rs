use std::{fs::*, io::Read};
use std::path::{Path};
use std::ffi::OsStr;
use std::env;
use colored::Colorize;

fn loadfile(path: &Path) -> std::io::Result<()> { 
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

                let mut contents = String::new();
                file.read_to_string(&mut contents)?;
                
                println!("{}", contents);
            } else{
                println!("{}" , format!("Error: Please provide a .pyowo file!").red().bold());
            }
        }
    }

    Ok(())

}
fn main() -> std::io::Result<()> {

    let args: Vec<String> = env::args().collect();

    if args.len() != 2{
        eprintln!("{}", format!("Error: Please provide a file to compile").red().bold());
        std::process::exit(0);
    }

    let path = Path::new(args[1].as_str());
    loadfile(path)?;
    Ok(())
}
 