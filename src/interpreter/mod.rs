//! Interprets SyntaxTree instances

use std::collections::HashMap;
use std::marker::PhantomData;
use cose_studio::Event;
use crate::interpreter::RuntimeValue::CoseSyntaxTree;
use crate::SyntaxTree;

/// Data about the running script
pub struct Runtime<'a> {
	syntax_tree: SyntaxTree,
	variables: HashMap<&'a str, RuntimeValue<'a>>
}

impl<'a> Runtime<'a> {
	fn new(syntax_tree: SyntaxTree) -> Self {
		Self {
			syntax_tree,
			variables: HashMap::new()
		}
	}

	fn init(&mut self) {
		self.variables.insert("parse", RuntimeValue::Function { body: PhantomData });
		self.variables.insert("run", RuntimeValue::Function { body: PhantomData });
	}

	pub fn get_variable(&self, name: &'a str) -> &RuntimeValue {
		&self.variables[name]
	}

	pub fn set_variable(&mut self, name: &'a str, value: RuntimeValue<'a>) {
		self.variables.insert(name, value);
	}

	pub fn call_function(&mut self, name: &'a str) {
		if name == "parse" {
			if let RuntimeValue::Function { .. } = self.get_variable(name) {
				let source = self.get_variable("cose_source");
				if let RuntimeValue::String(source) = *source {
					let counter = source.parse().unwrap();
					self.set_variable(
						"cose_syntax_tree",
						CoseSyntaxTree { counter }
					);
					return
				}
			}
		} else if name == "run" {
			if let RuntimeValue::Function { .. } = self.get_variable(name) {
				let syntax_tree = self.get_variable("cose_syntax_tree");
				if let RuntimeValue::CoseSyntaxTree { counter } = *syntax_tree {
					cose_studio::send_to_flutter(Event::SetCounter(counter));
					return
				}
			}
		}
		unimplemented!()
	}
}

pub enum RuntimeValue<'a> {
	Function {
		body: PhantomData<Vec<!>>
	},
	String(&'a str),
	CoseSyntaxTree { counter: u64 }
}

pub fn interpret<'a>(syntax_tree: SyntaxTree) -> Runtime<'a> {
	let mut runtime = Runtime::new(syntax_tree);
	runtime.init();
	runtime
}