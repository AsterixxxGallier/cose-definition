#![allow(unused, dead_code)]
#![feature(never_type)]

/// TODO
#[derive(Logos, Debug, PartialEq)]
enum Token {
	// Tokens can be literal strings, of any length.
	#[token("generate")]
	Generate,

	// Logos requires one token variant to handle errors,
	// it can be named anything you wish.
	#[error]
	// We can also use this variant to define whitespace,
	// or any other matches we wish to skip.
	#[regex(r"[ \t\n\f]+", logos::skip)]
	Error,
}

use logos::Logos;

pub struct SyntaxTree;

mod interpreter;

#[cfg(test)]
mod tests {
	use std::fs::File;
	use std::io::Read;

	use logos::Logos;
	use crate::interpreter::{interpret, RuntimeValue};

	use crate::{SyntaxTree, Token};

	#[test]
	fn interpreter() {
		let mut runtime = interpret(SyntaxTree);
		runtime.set_variable("cose_source", RuntimeValue::String("21"));
		runtime.call_function("parse");
		runtime.call_function("run");
	}

	#[test]
	fn lexer() {
		let mut file = File::open("core.definition.cose").unwrap();
		let mut content = String::new();
		file.read_to_string(&mut content).unwrap();
		let mut lex = Token::lexer(&content);

		assert_eq!(lex.next(), Some(Token::Generate));
		assert_eq!(lex.span(), 0..8);
		assert_eq!(lex.slice(), "generate");
	}
}
