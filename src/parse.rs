/*
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

type Expression = Equality;

type Equality = (Comparison, Option<MoreComparisons>);

type Comparison = (Addition, Option<MoreAdditions>);

enum MoreComparisons {
	NotEqual(Comparison, Option<Box<MoreComparisons>>),
	Equal(Comparison, Option<Box<MoreComparisons>>),
}

type Addition = (Multiplication, Option<MoreMultiplications>);

enum MoreAdditions {
	Greater(Addition, Option<Box<MoreAdditions>>),
	Less(Addition, Option<Box<MoreAdditions>>),
	GreaterEqual(Addition, Option<Box<MoreAdditions>>),
	LessEqual(Addition, Option<Box<MoreAdditions>>),
}

type Multiplication = (Unary, Option<MoreUnaries>);

enum MoreMultiplications {
	Minus(Multiplication, Option<Box<MoreMultiplications>>),
	Plus(Multiplication, Option<Box<MoreMultiplications>>),
}

enum Unary {
	Bang(Box<Unary>),
	Minus(Box<Unary>),
	Primary(Primary),
}

enum MoreUnaries {
	Divide(Unary, Option<Box<MoreUnaries>>),
	Times(Unary, Option<Box<MoreUnaries>>),
}

enum Primary {
	Number(f64),
	StringLiteral(String),
	False,
	True,
	Nil,
	Expression(Box<Expression>)
}