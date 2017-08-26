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
        write!(f, "({:?}, {})", self.token_type, self.lexeme)
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
		while self.ix + 1 < self.source_text.len() {
			self.start = self.ix;
			self.scan_token();
		}
		return &self.tokens;
	}

	fn scan_token(&mut self) {
		self.ix += 1;
	    let c = self.source_text.as_bytes()[self.ix];
	    match c {
			b'(' => { self.addToken(TokenType::LeftParen); },
			b')' => { self.addToken(TokenType::RightParen); },
			b'{' => { self.addToken(TokenType::LeftBrace); },
			b'}' => { self.addToken(TokenType::RightBrace); },
			b',' => { self.addToken(TokenType::Comma); },
			b'.' => { self.addToken(TokenType::Dot); },
			b'-' => { self.addToken(TokenType::Minus); },
			b'+' => { self.addToken(TokenType::Plus); },
			b';' => { self.addToken(TokenType::Semicolon); },
			b'*' => { self.addToken(TokenType::Star); },
			_ => {},
	    }
	}

	fn addToken(&mut self, token_type: TokenType) {
		self.tokens.push(Token::new(token_type, "", "", self.line));
	}
}