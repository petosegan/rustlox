use std::fmt;

#[derive(Debug)]
#[allow(dead_code)]
pub enum TokenType {
	// Single-character tokens.
	LeftParen, RightParen, LeftBrace, RightBrace,
	Comma, Dot, Minus, Plus, Semicolon, Slash, Star,

	// One or two character tokens.
	Bang, BangEqual,
	Equal, EqualEqual,
	Greater, GreaterEqual,
	Less, LessEqual,

	// Literals.
	Identifier, StringLiteral, Number,

	// Keywords.
	And, Class, Else, False, Fun, For, If, Nil, Or,
	Print, Return, Super, This, True, Var, While,

	Eof,
    Unknown,
}

#[derive(Debug)]
pub struct Token<'a> {
  token_type: TokenType,
  lexeme: &'a str,
  literal: &'a str,
  line: usize,
}
impl<'a> Token<'a> {
	pub fn new(token_type: TokenType, lexeme: &'a str, literal: &'a str, line: usize) -> Token<'a> {
		Token{ token_type: token_type, lexeme: lexeme, literal: literal, line: line}
	}
}

impl<'a> fmt::Display for Token<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({:?}, {}, ln {})", self.token_type, self.literal, self.line)
    }
}

pub struct Scanner<'a> {
	source_text: &'a str,
	tokens: Vec<Token<'a>>,
	ix: usize,
	start: usize,
	line: usize,
}

impl<'a> Scanner<'a> {
	pub fn new(source_text: &'a str) -> Scanner<'a> {
		Scanner { source_text: source_text, tokens: vec![] , ix: 0, start: 0, line: 1}
	}

	pub fn scan_tokens(&mut self) -> &Vec<Token<'a>> {
		while self.ix < self.source_text.len() {
            // ix is at start of next token
			self.start = self.ix;
			self.scan_token();
		}
		return &self.tokens;
	}

	fn scan_token(&mut self) {
	    let c = self.source_text.as_bytes()[self.ix];

        // handle comments and slash
        if c == b'/' {
            if self.match_next(b'/') {
                while self.peek() != b'\n' && self.ix + 1 < self.source_text.len() {
                    self.ix += 1;
                }
            } else {
                self.add_token(TokenType::Slash);
                self.ix+=1;
                return;
            }
        }

        // handle whitespace
        if c == b' ' || c == b'\t' || c == b'\r' {
            self.ix += 1;
            return;
        }
        if c == b'\n' {
            self.line += 1;
            self.ix += 1;
            return;
        }

        // handle strings
        if c == b'"' {
            self.scan_string();
            return;
        }

        // handle numbers
        if is_digit(c) {
            self.scan_number();
            return;
        }

	    let punct_token = match c {
			b'(' => TokenType::LeftParen,
			b'{' => TokenType::LeftBrace,
            b')' => TokenType::RightParen,
			b'}' => TokenType::RightBrace,
			b',' => TokenType::Comma,
			b'.' => TokenType::Dot,
			b'-' => TokenType::Minus,
			b'+' => TokenType::Plus,
			b';' => TokenType::Semicolon,
			b'*' => TokenType::Star,
			b'!' => {if self.match_next(b'=') {TokenType::BangEqual} else {TokenType::Bang}},
			b'=' => {if self.match_next(b'=') {TokenType::EqualEqual} else {TokenType::Equal}},
			b'<' => {if self.match_next(b'=') {TokenType::LessEqual} else {TokenType::Less}},
			b'>' => {if self.match_next(b'=') {TokenType::GreaterEqual} else {TokenType::Greater}},
			_ => TokenType::Unknown,
	    };
        self.add_token(punct_token);
        self.ix += 1;
	}

    fn peek(&self) -> u8 {
        if self.ix + 1 >= self.source_text.len() { return b'\0'; }
        return self.source_text.as_bytes()[self.ix + 1];
    }

    fn peek_next(&self) -> u8 {
        if self.ix + 2 >= self.source_text.len() {return b'\0'; }
        return self.source_text.as_bytes()[self.ix + 2];
    }

	fn add_token(&mut self, token_type: TokenType) {
		self.tokens.push(Token::new(token_type, "", "", self.line));
	}

    fn add_token_literal(&mut self, token_type: TokenType, literal: &'a str) {
        self.tokens.push(Token::new(token_type, "", literal, self.line));
    }

	fn match_next(&mut self, match_char: u8) -> bool {
		if self.ix + 1 >= self.source_text.len() { return false; }
		if self.source_text.as_bytes()[self.ix + 1] != match_char { return false; }

		self.ix += 1;
		return true;
	}

    fn scan_string(&mut self) {
        while self.peek() != b'"' && self.peek() != b'\0' {
            if self.peek() == b'\n' { self.line += 1; }
            self.ix += 1;
        }

        // closing "
        self.ix += 2;

        let lit_start = self.start + 1;
        let lit_end = self.ix - 1; 
        self.add_token_literal(TokenType::StringLiteral, &self.source_text[lit_start..lit_end]);
    }

    fn scan_number(&mut self) {
        println!("scanning number at line {}", self.line);
        while is_digit(self.source_text.as_bytes()[self.ix]) {
            println!("saw digit at {}", self.ix);
            self.ix += 1;
        }
        if self.source_text.as_bytes()[self.ix] == b'.' && is_digit(self.peek()) {
            self.ix += 1;

            while is_digit(self.source_text.as_bytes()[self.ix]) {
                self.ix += 1;
            }
        }

        let num_start = self.start;
        let num_end = self.ix;
        self.add_token_literal(TokenType::Number, &self.source_text[num_start..num_end]);
    }
}

fn is_digit(c: u8) -> bool {
    c >= b'0' && c <= b'9'
}
