/*

Expression Grammar
==================
expression → literal
           | unary
           | binary
           | grouping ;

literal    → NUMBER | STRING | "true" | "false" | "nil" ;
grouping   → "(" expression ")" ;
unary      → ( "-" | "!" ) expression ;
binary     → expression operator expression ;
operator   → "==" | "!=" | "<" | "<=" | ">" | ">="
           | "+"  | "-"  | "*" | "/" ;

Precedence Grammar
==================
expression     → equality ;
equality       → comparison ( ( "!=" | "==" ) comparison )* ;
comparison     → addition ( ( ">" | ">=" | "<" | "<=" ) addition )* ;
addition       → multiplication ( ( "-" | "+" ) multiplication )* ;
multiplication → unary ( ( "/" | "*" ) unary )* ;
unary          → ( "!" | "-" ) unary ;
               | primary ;
primary        → NUMBER | STRING | "false" | "true" | "nil"
               | "(" expression ")" ;
*/

use scanner::{TokenType, Token};

#[derive(Debug)]
pub enum Expression {
	Number(f64),
	Literal(String),
	True,
	False,
	Nil,
	Unary(TokenType, Box<Expression>),
	Binary(Box<Expression>, TokenType, Box<Expression>),
	Grouping(Box<Expression>),
}


pub struct Parser<'a> {
	tokens: Vec<Token<'a>>,
	current: usize,
}

impl <'a> Parser <'a> {
	pub fn new(tokens: Vec<Token<'a>>) -> Parser<'a> {
		Parser {tokens: tokens, current: 0}
	}

	pub fn expression(&mut self) -> Result<Expression, ()> {
		return self.equality();
	}

	fn equality(&mut self) -> Result<Expression, ()> {
		let mut expr = self.comparison()?;

		while self.match_types(vec![TokenType::BangEqual, TokenType::EqualEqual]) {
			let operator = self.previous().token_type();
			let right = self.comparison()?;
			expr = Expression::Binary {0: Box::new(expr), 1: operator, 2: Box::new(right)};
		}

		return Ok(expr);
	}

	fn comparison(&mut self) -> Result<Expression, ()> {
		let mut expr = self.addition()?;

		while self.match_types(vec![TokenType::Greater, TokenType::GreaterEqual, TokenType::Less, TokenType::LessEqual]) {
			let operator = self.previous().token_type();
			let right = self.addition()?;
			expr = Expression::Binary {0: Box::new(expr), 1: operator, 2: Box::new(right)};
		}

		return Ok(expr);
	}

	fn addition(&mut self) -> Result<Expression, ()> {
		let mut expr = self.multiplication()?;

		while self.match_types(vec![TokenType::Minus, TokenType::Plus]) {
			let operator = self.previous().token_type();
			let right = self.multiplication()?;
			expr = Expression::Binary {0: Box::new(expr), 1: operator, 2: Box::new(right)};
		}

		return Ok(expr);
	}

	fn multiplication(&mut self) -> Result<Expression, ()> {
		let mut expr = self.unary()?;

		while self.match_types(vec![TokenType::Slash, TokenType::Star]) {
			let operator = self.previous().token_type();
			let right = self.unary()?;
			expr = Expression::Binary {0: Box::new(expr), 1: operator, 2: Box::new(right)};
		}

		return Ok(expr);
	}

	fn unary(&mut self) -> Result<Expression, ()> {
		if self.match_types(vec![TokenType::Bang, TokenType::Minus]) {
			let operator = self.previous().token_type();
			let right = self.unary()?;
			return Ok(Expression::Unary {0: operator, 1: Box::new(right)});
		}
		return self.primary();
	}

	fn primary(&mut self) -> Result<Expression, ()> {
		if self.match_types(vec![TokenType::False]) { return Ok(Expression::False); }
		if self.match_types(vec![TokenType::True])  { return Ok(Expression::True);  }
		if self.match_types(vec![TokenType::Nil])   { return Ok(Expression::Nil);   }

		if self.match_types(vec![TokenType::StringLiteral]) {
			return Ok(Expression::Literal {0: self.previous().literal() });
		}
		if self.match_types(vec![TokenType::Number]) {
			return Ok(Expression::Number {0: self.previous().literal().parse::<f64>().unwrap()})
		}

		if self.match_types(vec![TokenType::LeftParen]) {
			let expr = self.expression()?;
			self.consume(TokenType::RightParen)?;
			return Ok(Expression::Grouping {0: Box::new(expr)});
		}

		Err(())
	}

	fn match_types(&mut self, types: Vec<TokenType>) -> bool {
		for token_type in types {
			if self.check(token_type) {
				self.advance();
				return true;
			}
		}
		return false;
	}

	fn check(&self, token_type: TokenType) -> bool {
		if self.is_at_end() {return false;}
		self.peek().token_type() == token_type
	}

	fn is_at_end(&self) -> bool {
		self.peek().token_type() == TokenType::Eof
	}

	fn consume(&mut self, token_type: TokenType) -> Result<&Token<'a>, ()> {
		if self.check(token_type) { return Ok(self.advance()); }
		Err(())
	}

	fn advance(&mut self) -> &Token<'a> {
		if !self.is_at_end() {self.current += 1; }
		self.previous()
	}

	fn previous(&self) -> &Token<'a> {
		&(self.tokens[self.current - 1])
	}

	fn peek(&self) -> &Token<'a> {
		&(self.tokens[self.current])
	}
}