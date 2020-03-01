use tower_lsp::lsp_types::*;
use tree_sitter::Node;
use super::statements;
use super::helpers::*;
use std::collections::HashMap;
use crate::script_commands::ScriptCommand;

// Debugger
use std::io::prelude::*;
use std::net::TcpStream;

pub fn format(
	_dbg: &mut TcpStream,
	node: &Node,
	code: &String,
	formatter_info: &mut (u64, u64),
	indent_level: u8,
	commands: &HashMap<String, ScriptCommand>,
	edits: &mut Vec<TextEdit>,
) {
	let mut cursor = node.walk();
	cursor.goto_first_child(); // TODO: Maybe add handling for safety
	// first child IS a '{', already written by the parent
	let parent_indent = std::iter::repeat("\t").take((indent_level) as usize).collect::<String>();

	while cursor.goto_next_sibling() {
		let node = cursor.node();
		match node.kind().to_lowercase().as_str() {
			"}" => {
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
					new_text: format!(
						"{}}}\n",
						parent_indent
					),
				});
			
				formatter_info.0 += 1;
				formatter_info.1 = 0;
			},
			"expression_statement" => {
				let mut stmt_cursor = node.walk();
				stmt_cursor.goto_first_child();
				if stmt_cursor.node().kind().eq_ignore_ascii_case("identifier") {
					let txt = get_node_text(&stmt_cursor.node(), code);
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
							new_text: format!(
								"{}\t{}();\n",
								parent_indent, txt
							),
						});
					
						formatter_info.0 += 1;
						formatter_info.1 = 0;
					}
				}
			},
			"old_function" => statements::old_function::format(_dbg, &node, code, formatter_info, indent_level, edits),
			_ => continue, // TODO: This may make text disappear
		}
	}
}
