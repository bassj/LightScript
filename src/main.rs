use std::env;
use std::fs;
use std::error::Error;


fn print_usage(command: &str) {
    println!("USAGE: {} source_files", command);
}

fn main() ->  Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage(args[0].as_ref());
    }

    let file_paths: &[String] = &args[1..];

    for path in file_paths {
        let src = fs::read_to_string(path)?;

        
        println!("{}", src);
    }


    Ok(())
}
