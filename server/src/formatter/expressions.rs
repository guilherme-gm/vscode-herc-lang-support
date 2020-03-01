use tower_lsp::lsp_types::*;
use tree_sitter::Node;

// Debugger
use std::io::prelude::*;
use std::net::TcpStream;

pub mod assignment_expression;
pub mod binary_expression;
pub mod conditional_expression;
pub mod exp_false;
pub mod exp_number_literal;
pub mod exp_string;
pub mod exp_true;
pub mod function_expression;
pub mod identifier;
pub mod parenthesized_expression;
pub mod scoped_identifier;
pub mod subscript_expression;
pub mod unary_expression;
pub mod update_expression;

pub fn resolve(
	_dbg: &mut TcpStream,
	node: &Node,
	code: &String,
	formatter_info: &mut (u64, u64),
	indent_level: u8,
	edits: &mut Vec<TextEdit>,
) -> bool {
	match node.kind().to_lowercase().as_str() {
		"conditional_expression" => {
			conditional_expression::format(_dbg, node, code, formatter_info, edits)
		}
		"assignment_expression" => {
			assignment_expression::format(_dbg, node, code, formatter_info, edits)
		}
		"unary_expression" => unary_expression::format(_dbg, node, code, formatter_info, edits),
		"binary_expression" => binary_expression::format(_dbg, node, code, formatter_info, edits),
		"false" => exp_false::format(_dbg, node, code, formatter_info, edits),
		"true" => exp_true::format(_dbg, node, code, formatter_info, edits),
		"number_literal" => exp_number_literal::format(_dbg, node, code, formatter_info, edits),
		"string" => exp_string::format(_dbg, node, code, formatter_info, indent_level, edits),
		"function_expression" => {
			function_expression::format(_dbg, node, code, formatter_info, edits)
		}
		"subscript_expression" => {
			subscript_expression::format(_dbg, node, code, formatter_info, edits)
		}
		"update_expression" => update_expression::format(_dbg, node, code, formatter_info, edits),
		"identifier" => identifier::format(_dbg, node, code, formatter_info, edits),
		"scoped_identifier" => scoped_identifier::format(_dbg, node, code, formatter_info, edits),
		"parenthesized_expression" => {
			parenthesized_expression::format(_dbg, node, code, formatter_info, edits)
		}
		_ => return false,
	}

	true
}

pub fn resolve_stmt(
	_dbg: &mut TcpStream,
	node: &Node,
	code: &String,
	formatter_info: &mut (u64, u64),
	indent_level: u8,
	edits: &mut Vec<TextEdit>,
) -> bool {
	let parent_indent = str::repeat("\t", indent_level as usize);
	edits.push(TextEdit {
		range: Range {
			start: Position {
				line: formatter_info.0,
				character: formatter_info.1,
			},
			end: Position {
				line: formatter_info.0,
				character: formatter_info.1 + (indent_level as u64) + 1,
			},
		},
		new_text: format!("\t{}", parent_indent),
	});
	formatter_info.1 += (indent_level as u64) + 1;

	let result = resolve(_dbg, node, code, formatter_info, indent_level, edits);

	edits.push(TextEdit {
		range: Range {
			start: Position {
				line: formatter_info.0,
				character: formatter_info.1,
			},
			end: Position {
				line: formatter_info.0 + 1,
				character: 0,
			},
		},
		new_text: String::from(";\n"),
	});
	formatter_info.0 += 1;
	formatter_info.1 = 0;

	result
}
