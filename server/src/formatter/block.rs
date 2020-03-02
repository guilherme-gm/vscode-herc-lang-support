use tower_lsp::lsp_types::*;
use tree_sitter::Node;
use super::statements;
use super::helpers::*;
use super::expressions;
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
	let parent_indent = std::iter::repeat("\t").take((indent_level) as usize).collect::<String>();
	edits.push(get_singleline_edit(String::from("{\n"), formatter_info, true));

	while cursor.goto_next_sibling() {
		let node = cursor.node();
		debug_!(_dbg, format!("> block::node: {:?}", node));
		if node.kind().to_lowercase().eq_ignore_ascii_case("}") {
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
		}

		statements::resolve(_dbg, &node, code, formatter_info, indent_level, commands, edits);
	}
}
