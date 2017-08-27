use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;

mod scanner;
mod parse;
mod interpret;

struct RustLox {
	// need to be able to maintain intepreter state
	// between lines in REPL
	interpreter: interpret::Interpreter,
}

impl RustLox {
	pub fn new() -> Self {
		RustLox { interpreter: interpret::Interpreter::new() }
	}

	fn run(&mut self, lines: &str) {
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
			return;
		}
		// {
		// 	let parsed = this_parse.unwrap();
		// 	println!("\nParses to:\n{:?}", parsed);
		// }

		let this_result = self.interpreter.interpret(this_parse.unwrap());
		if let Err(e) = this_result {
			println!("Runtime error: {:?}", e);
			return;
		}
		// println!("\nInterprets to: {:?}", this_result.unwrap());
	}

	fn run_prompt(&mut self) {
		loop {
			print!("> ");
			io::stdout().flush().expect("Error flushing stdout");

			let mut line = String::new();

		    io::stdin().read_line(&mut line)
		        .expect("Failed to read line");

		    self.run(&line);
		}
	}

	fn run_file(&mut self, filename: &str) {
		let mut f = File::open(filename).expect("file not found");

	    let mut contents = String::new();
	    f.read_to_string(&mut contents).expect("something went wrong reading the file");

	    self.run(&contents);
	}
}

pub fn main() {
    let args: Vec<String> = env::args().collect();

    let mut this_rustlox = RustLox::new();

    match args.len() {
    	1 => { this_rustlox.run_prompt(); },
    	2 => { this_rustlox.run_file(&args[1]); },
    	_ => { println!("Usage: rustlox [script]"); },
    }
}
