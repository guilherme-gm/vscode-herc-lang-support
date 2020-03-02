use tower_lsp::lsp_types::*;
use tree_sitter::Node;
use super::block;
use super::expressions;
use super::helpers::*;
use std::collections::HashMap;
use crate::script_commands::ScriptCommand;


// Debugger
use std::io::prelude::*;
use std::net::TcpStream;

pub mod if_statement;
pub mod old_function;
pub mod return_stmt;
pub mod break_stmt;
pub mod continue_stmt;
pub mod goto_stmt;
pub mod end_stmt;
pub mod function_decl;
pub mod function_stmt;
pub mod for_statement;
pub mod while_statement;
pub mod do_while_statement;
pub mod switch_statement;
pub mod case_statement;
pub mod labeled_statement;

// $.block, -- implemented in parent
// $.expression_statement, -- Implemented here

pub fn resolve(
	_dbg: &mut TcpStream,
	node: &Node,
	code: &String,
	formatter_info: &mut (u64, u64),
	indent_level: u8,
	commands: &HashMap<String, ScriptCommand>,
	edits: &mut Vec<TextEdit>,
) -> bool {
	debug_!(_dbg, format!("> resolve_stmt: {:?}", node));
	let parent_indent = str::repeat("\t", indent_level as usize);

	match node.kind().to_lowercase().as_str() {
		"if_statement" => {
			if_statement::format(_dbg, node, code, formatter_info, indent_level, commands, edits);
		}
		"for_statement" => {
			for_statement::format(_dbg, node, code, formatter_info, indent_level, commands, edits);
		}
		"while_statement" => {
			while_statement::format(_dbg, node, code, formatter_info, indent_level, commands, edits);
		}
		"do_while_statement" => {
			do_while_statement::format(_dbg, node, code, formatter_info, indent_level, commands, edits);
		}
		"switch_statement" => {
			switch_statement::format(_dbg, node, code, formatter_info, indent_level, commands, edits);
		}
		"case_statement" => {
			case_statement::format(_dbg, node, code, formatter_info, indent_level, commands, edits);
		}
		"labeled_statement" => {
			labeled_statement::format(_dbg, node, code, formatter_info, indent_level, commands, edits);
		}
		"expression_statement" => {
			let mut stmt_cursor = node.walk();
			stmt_cursor.goto_first_child();
			let exp_node = stmt_cursor.node();
			if exp_node.kind().eq_ignore_ascii_case("identifier") {
				let txt = get_node_text(&exp_node, code);
				if commands.contains_key(&txt) {
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
						new_text: format!("{}\t{}();\n", parent_indent, txt),
					});
					formatter_info.0 += 1;
					formatter_info.1 = 0;
				} else {
					expressions::resolve_stmt(
						_dbg,
						&exp_node,
						code,
						formatter_info,
						indent_level,
						edits,
					);
				}
			} else {
				expressions::resolve_stmt(
					_dbg,
					&exp_node,
					code,
					formatter_info,
					indent_level,
					edits,
				);
			}
		}
		"block" => {
			block::format(_dbg, &node, code, formatter_info, indent_level, commands, edits);
		}
		"return_statement" => {
			return_stmt::format(_dbg, &node, code, formatter_info, indent_level, edits);
		}
		"break_statement" => {
			break_stmt::format(_dbg, &node, code, formatter_info, indent_level, edits);
		}
		"continue_statement" => {
			continue_stmt::format(_dbg, &node, code, formatter_info, indent_level, edits);
		}
		"goto_statement" => {
			goto_stmt::format(_dbg, &node, code, formatter_info, indent_level, edits);
		}
		"end_statement" => {
			end_stmt::format(_dbg, &node, code, formatter_info, indent_level, edits);
		}
		"function_declaration" => {
			function_decl::format(_dbg, &node, code, formatter_info, indent_level, commands, edits);
		}
		"function_statement" => {
			function_stmt::format(_dbg, &node, code, formatter_info, indent_level, edits);
		}
		"old_function" => {
			old_function::format(_dbg, &node, code, formatter_info, indent_level, edits);
		}
		_ => return false,
	}

	true
}
