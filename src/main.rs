use std::{env, error::Error, fs, fs::File, io::Write};

mod lex;
use lex::{Token, TokenStream};

mod emit;
use emit::ModuleEmitter;

mod parse;
use parse::Parser;

fn print_usage(command: &str) {
    println!("USAGE: {} source_files", command);
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage(args[0].as_ref());
    }

    let file_paths: &[String] = &args[1..];

    for path in file_paths {
        let src = fs::read_to_string(path)?;

        let tokstrm = TokenStream::new(&src);

        let mut parser = Parser::new(tokstrm);

        let parse_result = parser.parse();

        match parse_result {
            Err(e) => {
                println!("ERR");
            },
            _ => {
                println!("goodnight!");
            }
        }

    }

    //let emitter = ModuleEmitter;

    //let bincode = emitter.emit();

    //println!("{:?}", bincode);

    //let mut out = File::create("a.wasm")?;
    //out.write(&bincode)?;
    Ok(())
}
