use parse::{Expression, Statement};
use scanner::TokenType;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
	Number(f64),
	StringLiteral(String),
	Boolean(bool),
	Nil,
}

pub struct Interpreter {
	environment: HashMap<String, Value>,
}

impl Interpreter {
	pub fn new() -> Self {
		Interpreter { environment: HashMap::new() }
	}

	pub fn interpret(&mut self, program: Vec<Statement>) -> Result<(), &'static str> {
	    for stmt in program {
	        self.execute(stmt)?;
	    }
	    Ok(())
	}

	fn execute(&mut self, stmt: Statement) -> Result<(), &'static str>{
	    match stmt {
	        Statement::ExprStmt(e) => { self.evaluate(e)?; },
	        Statement::PrintStmt(e) => {
	            let val = self.evaluate(e)?;
	            println!("{:?}", val);
	        },
	        Statement::VarDecl(var_name, initializer) => {
	        	let mut var_val = Value::Nil;
	        	if initializer != Expression::Nil {
	        		var_val = self.evaluate(initializer)?;
	        	}

	        	self.environment.insert(var_name, var_val);
	        },
	    }
	    Ok(())
	}

	fn evaluate(&mut self, expr: Expression) -> Result<Value, &'static str> {
		match expr {
			Expression::Number(n) => Ok(Value::Number(n)),
			Expression::Literal(s) => Ok(Value::StringLiteral(s)),
			Expression::True => Ok(Value::Boolean(true)),
			Expression::False => Ok(Value::Boolean(false)),
			Expression::Nil => Ok(Value::Nil),
			Expression::Unary(tt, be) => self.evaluate_unary(tt, *be),
			Expression::Binary(bel, tt, ber) => self.evaluate_binary(*bel, tt, *ber),
			Expression::Grouping(be) => self.evaluate(*be),
	        Expression::Variable(var_name) => self.var_lookup(var_name),
	        Expression::Assign(var_name, be) => {
	        	if self.environment.contains_key(&var_name) {
	        		let var_val = self.evaluate(*be)?;
	        		self.environment.insert(var_name, var_val.clone());
	        		return Ok(var_val);
	        	} else {
	        		return Err("Undefined variable");
	        	}},
		}
	}

	fn var_lookup(&self, var_name: String) -> Result<Value, &'static str> {
		if let Some(var_val) = self.environment.get(&var_name) {
			let owned_val = (*var_val).clone();
			return Ok(owned_val);
		}
		return Err("undefined variable");
	}

	fn evaluate_unary(&mut self, operator: TokenType, expr: Expression) -> Result<Value, &'static str> {
		match operator {
			TokenType::Bang => {
				let expr_value = self.evaluate(expr)?;
				return Ok(Value::Boolean(!is_truthy(expr_value)));
			},
			TokenType::Minus => {
				let expr_value = self.evaluate(expr)?;
				if let Value::Number(n) = expr_value {
					return Ok(Value::Number(-n));
				} else {
					return Err("attempted negation on non-number");
				}
			}
			_ => { return Err("unrecognized unary operator"); }
		}
	}

	fn evaluate_binary(&mut self, expr_l: Expression, operator: TokenType, expr_r: Expression) -> Result<Value, &'static str> {
		let val_l = self.evaluate(expr_l)?;
		let val_r = self.evaluate(expr_r)?;
		match operator {
			TokenType::EqualEqual => Ok(Value::Boolean(val_l == val_r)),
			TokenType::BangEqual => Ok(Value::Boolean(val_l != val_r)),
			TokenType::Less => {return number_comp(val_l, val_r, |x, y| x < y); }
			TokenType::LessEqual => {return number_comp(val_l, val_r, |x, y| x <= y); }
			TokenType::Greater => {return number_comp(val_l, val_r, |x, y| x > y); }
			TokenType::GreaterEqual => {return number_comp(val_l, val_r, |x, y| x >= y); }
			TokenType::Plus => {return arith_op(val_l, val_r, |x, y| x + y); }
			TokenType::Minus => {return arith_op(val_l, val_r, |x, y| x - y); }
			TokenType::Star => { return arith_op(val_l, val_r, |x, y| x * y); }
			TokenType::Slash => { return arith_op(val_l, val_r, |x, y| x / y); }
			_ => Err("unrecognized binary operator"),
		}
	}
}

fn arith_op<F>(val_r: Value, val_l: Value, op: F) -> Result<Value, &'static str> 
	where F: Fn(f64, f64) -> f64 {
	if let Value::Number(n_l) = val_l {
		if let Value::Number(n_r) = val_r {
			return Ok(Value::Number(op(n_l, n_r)));
		}
	}
	return Err("attempted arithmetic with non-number");
}

fn number_comp<F>(val_r: Value, val_l: Value, op: F) -> Result<Value, &'static str> 
	where F: Fn(f64, f64) -> bool {
	if let Value::Number(n_l) = val_l {
		if let Value::Number(n_r) = val_r {
			return Ok(Value::Boolean(op(n_l, n_r)));
		}
	}
	return Err("attempted comparison with non-number");
}

fn is_truthy(val: Value) -> bool {
	match val {
		Value::Boolean(false) | Value:: Nil => false,
		_ => true,
	}
}