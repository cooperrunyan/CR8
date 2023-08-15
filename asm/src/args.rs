use std::{env, fs};

pub fn parse() -> Result<(String, String, String), String> {
    let args: Vec<_> = env::args().collect();

    let mut input_path = String::new();
    let mut output_path = String::new();

    for (i, arg) in args.iter().enumerate() {
        if arg == "-i" {
            if args.len() <= i + 1 {
                break;
            }
            input_path = args[i + 1].to_string();
        } else if arg == "-o" {
            if args.len() <= i + 1 {
                break;
            }
            output_path = args[i + 1].to_string();
        }
    }

    if input_path.is_empty() {
        return Err("Expected input file".to_string());
    }

    if output_path.is_empty() {
        return Err("Expected output file".to_string());
    }

    let input = match fs::read(input_path.clone()) {
        Ok(i) => match String::from_utf8(i) {
            Ok(s) => s,
            Err(_) => return Err("Bad input file".to_string()),
        },
        Err(_) => return Err("Could not read input file".to_string()),
    };

    Ok((input, input_path, output_path))
}
