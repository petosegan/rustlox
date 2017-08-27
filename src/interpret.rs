use parse::Expression;
use scanner::TokenType;

#[derive(Debug, PartialEq)]
pub enum Value {
	Number(f64),
	StringLiteral(String),
	Boolean(bool),
	Nil,
}

pub fn interpret(expr: Expression) -> Result<Value, ()> {
	match expr {
		Expression::Number(n) => Ok(Value::Number(n)),
		Expression::Literal(s) => Ok(Value::StringLiteral(s)),
		Expression::True => Ok(Value::Boolean(true)),
		Expression::False => Ok(Value::Boolean(false)),
		Expression::Nil => Ok(Value::Nil),
		Expression::Unary(tt, be) => interpret_unary(tt, *be),
		Expression::Binary(bel, tt, ber) => interpret_binary(*bel, tt, *ber),
		Expression::Grouping(be) => interpret(*be),
	}
}

fn interpret_unary(operator: TokenType, expr: Expression) -> Result<Value, ()> {
	match operator {
		TokenType::Bang => {
			let expr_value = interpret(expr)?;
			return Ok(Value::Boolean(!is_truthy(expr_value)));
		},
		TokenType::Minus => {
			let expr_value = interpret(expr)?;
			if let Value::Number(n) = expr_value {
				return Ok(Value::Number(-n));
			} else {
				return Err(());
			}
		}
		_ => { return Err(()); }
	}
}

fn interpret_binary(expr_l: Expression, operator: TokenType, expr_r: Expression) -> Result<Value, ()> {
	let val_l = interpret(expr_l)?;
	let val_r = interpret(expr_r)?;
	match operator {
		TokenType::EqualEqual => Ok(Value::Boolean(val_l == val_r)),
		TokenType::BangEqual => Ok(Value::Boolean(val_l != val_r)),
		TokenType::Less => {return number_comp(val_l, val_r, |x, y| x < y); }
		TokenType::LessEqual => {return number_comp(val_l, val_r, |x, y| x <= y); }
		TokenType::Greater => {return number_comp(val_l, val_r, |x, y| x > y); }
		TokenType::GreaterEqual => {return number_comp(val_l, val_r, |x, y| x >= y); }
		TokenType::Plus => {return arith_op(val_l, val_r, |x, y| x / y); }
		TokenType::Minus => {return arith_op(val_l, val_r, |x, y| x / y); }
		TokenType::Star => { return arith_op(val_l, val_r, |x, y| x * y); }
		TokenType::Slash => { return arith_op(val_l, val_r, |x, y| x / y); }
		_ => Err(()),
	}
}

fn arith_op<F>(val_r: Value, val_l: Value, op: F) -> Result<Value, ()> 
	where F: Fn(f64, f64) -> f64 {
	if let Value::Number(n_l) = val_l {
		if let Value::Number(n_r) = val_r {
			return Ok(Value::Number(op(n_l, n_r)));
		}
	}
	return Err(());
}

fn number_comp<F>(val_r: Value, val_l: Value, op: F) -> Result<Value, ()> 
	where F: Fn(f64, f64) -> bool {
	if let Value::Number(n_l) = val_l {
		if let Value::Number(n_r) = val_r {
			return Ok(Value::Boolean(op(n_l, n_r)));
		}
	}
	return Err(());
}

fn is_truthy(val: Value) -> bool {
	match val {
		Value::Boolean(false) | Value:: Nil => false,
		_ => true,
	}
}