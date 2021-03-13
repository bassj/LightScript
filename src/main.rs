use std::{env, error::Error, fs};

mod lex;
use lex::{Token, TokenStream};

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

        let tokens = TokenStream::new(&src);

        for tok in tokens {
            match tok {
                Token::IntLiteral(value) => {
                    println!("Integer with value: {}", value);
                }
                Token::Operator(op) => {
                    println!("Operator: {:?}", op);
                }
                Token::Word(word) => {
                    println!("Word: {}", word);
                }
                Token::Stop => {
                    println!("Line Stop: ;");
                }
                _ => {
                    println!("Other Token");
                }
            }
        }
    }

    Ok(())
}
