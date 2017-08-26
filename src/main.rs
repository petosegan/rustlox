use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;

mod scanner;
mod parse;

fn run(lines: &str) {
	let mut this_scanner = scanner::Scanner::new(lines);
	println!("Should run: \n{}", lines);
	let scanned_tokens = this_scanner.scan_tokens();
	for token in scanned_tokens {
		println!("{}", token);
	}
	// println!("Yielding tokens: {:?}", &(this_scanner.scan_tokens()));
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

    println!("Should run file {}.", filename);
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
