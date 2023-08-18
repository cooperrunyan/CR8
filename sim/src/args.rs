use std::{env, fs};

pub fn parse() -> Result<Vec<u8>, String> {
    let args: Vec<_> = env::args().collect();

    let mut file = String::new();

    for (i, arg) in args.iter().enumerate() {
        if arg == "-i" {
            if args.len() <= i + 1 {
                break;
            }
            file = args[i + 1].to_string();
        }
    }

    if file.is_empty() {
        return Err("Expected input file".to_string());
    }

    match fs::read(file.clone()) {
        Ok(i) => Ok(i),
        Err(_) => Err("Could not read input file".to_string()),
    }
}
