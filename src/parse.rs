/*
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
*/

enum Expression {
	Literal(Literal),
	Unary(Unary),
	Binary(Box<Expression>, Operator, Box<Expression>),
	Grouping(Box<Expression>),
}

enum Literal {
	Number(f64),
	StringLiteral(String),
	True,
	False,
	Nil,
}

enum Unary {
	Minus(Box<Expression>),
	Bang(Box<Expression>),
}

enum Operator {
	EqualEqual,
	BangEqual,
	Less,
	LessEqual,
	Greater,
	GreaterEqual,
	Plus,
	Minus,
	Star,
	Slash,
}