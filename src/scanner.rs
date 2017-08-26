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
pub struct Token {
  token_type: TokenType,
  lexeme: String,
  //Object literal;
  line: u32,
}
impl Token {
	pub fn new(token_type: TokenType, lexeme: String, line: u32) -> Self {
		Token{ token_type: token_type, lexeme: lexeme, line: line}
	}
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({:?}, {})", self.token_type, self.lexeme)
    }
}

pub fn scan_tokens(text: &str) -> Vec<Token> {
	vec![Token::new(TokenType::LeftParen, "(".to_string(), 0),
	     Token::new(TokenType::RightParen, ")".to_string(), 1)]
}