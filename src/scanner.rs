#[derive(Debug)]
pub enum Token {
	TokenA,
	TokenB,
}

pub fn scan_tokens(text: &str) -> Vec<Token> {
	vec![Token::TokenA, Token::TokenB]
}