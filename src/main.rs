use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;

mod scanner;
mod parse;
mod interpret;

fn run(lines: &str) {
	let this_scanner = scanner::Scanner::new(lines);
	// println!("Should run: \n{}", lines);
	let scanned_tokens = this_scanner.scan_tokens();
	// for token in &scanned_tokens {
	// 	println!("{}", token);
	// }
	let mut this_parser = parse::Parser::new(scanned_tokens);
	let this_parse = this_parser.parse();
	if let Err(e) = this_parse {
		println!("Parse error: {:?}", e);
	}
	// println!("\nParses to:\n{:?}", this_exp);

	let this_interpret = interpret::interpret(this_parse.unwrap());
	if let Err(e) = this_interpret {
		println!("Runtime error: {:?}", e);
	}
	// println!("\nInterprets to: {:?}", this_value);
	// println!("{:?}", this_value);
}

fn run_prompt() {
	loop {
		print!("> ");
		io::stdout().flush().expect("Error flushing stdout");

		let mut line = String::new();

	    io::stdin().read_line(&mut line)
	        .expect("Failed to read line");

	    run(&line);
	}
}

fn run_file(filename: &str) {
	let mut f = File::open(filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("something went wrong reading the file");

    run(&contents);
}

pub fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
    	1 => { run_prompt(); },
    	2 => { run_file(&args[1]); },
    	_ => { println!("Usage: rustlox [script]"); },
    }
}
