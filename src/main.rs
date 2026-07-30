#![allow(unused_variables)]

//-------------------
// declare imports 
//------------------

use std::env;
use std::fs;
use std::process::ExitCode;
use std::collections::HashMap;

mod types;
mod parser;
mod interpreter;
mod tokenizer;
mod statements;

use types::*;
use parser::*;
use interpreter::*;
use tokenizer::*;
use statements::*;


//-----------------------
// define main function with 3 cmds
// 1. tokenize file ex. ['STRING HELLO null', 'COMMA , null', 'IDENTIFIER WORLD null']
// 2. parse generated AST 
// 3. Interpret generated AST
//----------------------

fn main() -> ExitCode {
    //--------------------------------
    // get command and file contents 
    // for parsing 
    //--------------------------------

    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} tokenize <filename>", args[0]);
        return ExitCode::from(1);
    }

    let command = &args[1];
    let filename = &args[2];
    let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
        eprintln!("Failed to read file {}", filename);
        String::new()
    });

    match command.as_str() {
        "tokenize" =>{
            let (tokens , err) = tokenize(file_contents);
            for token in &tokens{ // reference rather than borrowing it
                println!("{}", token);
            };
            if !err.is_empty() {
                for err_message in err{
                   eprintln!("{}", err_message);
                };
                return ExitCode::from(65)
            };
            return ExitCode::from(0)
           
        },"parse" => {
            let (tokens, _) = tokenize(file_contents); 
            if tokens.len() == 1{
                return ExitCode::from(65)
            }

            let mut parser: Parser = Parser{tokens, current: 0};
            let result = match parser.equality(){
                Ok(val) => {
                    let parsed_tree: String = parse(val);
                    println!("{}", parsed_tree);
                }
                Err(e) => {
                    eprintln!("{}", e);
                    return ExitCode::from(65)
                }  
            };
            return ExitCode::from(0)
        },"evaluate" => {
            let (tokens, err_str) = tokenize(file_contents); // NUMBER 50 50.0, EOF null
            let mut parser = Parser{tokens, current: 0};
            let result = match parser.equality(){
                Ok(val) => { 
                    let mut interpreter: Interpreter = Interpreter{scope: vec![HashMap::new()], current_scope: 0};
                    let res = match interpreter.evaluate(val){
                        Ok(val)=> println!("{}", val),
                        Err(err) =>{
                            eprintln!("{}", err); // print expression
                            return ExitCode::from(70)
                        }
                    };
                },
                Err(e) => {
                    eprintln!("{}", e);
                    return ExitCode::from(65) 
                } 
            };
            return ExitCode::from(0)
        },"run"=>{
            let (tokens, err_str) = tokenize(file_contents); // NUMBER 50 50.0, EOF null
            let mut parser = Parser{tokens, current: 0};
            match parser.declaration(){
                Ok(val)=>{
                    let mut scope: HashMap<String, Lit> = HashMap::new();
                    scope.insert("clock".to_string(), Lit::NativeFn("clock".to_string()));
                    let mut interpreter: Interpreter = Interpreter{scope: vec![scope], current_scope:0};

                    match execute(val, &mut interpreter){
                        Ok(val) => {},
                        Err(e) => {
                            eprintln!("{}", e);
                            return ExitCode::from(70)
                        }
                    }
                },
                Err(e)=>{
                    eprintln!("{}", e);
                    return ExitCode::from(65)
                }
            };
            return ExitCode::from(0)
        }, _ => {
            eprintln!("Unknown command: {}", command);
            return ExitCode::from(1)
        }
    }
}
