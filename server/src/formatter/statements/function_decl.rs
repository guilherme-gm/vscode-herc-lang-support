use super::super::helpers::*;
use tower_lsp::lsp_types::*;
use tree_sitter::Node;
use super::super::block;
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
	debug_!(_dbg, format!("> function_decl: {:?}", node));
	let mut cursor = node.walk();
	cursor.goto_first_child(); // TODO: Maybe add handling for safety
    let name = get_next_named(&mut cursor, code, "name").unwrap();
    goto_name(&mut cursor, "body");
	
	let parent_indent = str::repeat("\t", 1 + indent_level as usize);
	edits.push(get_singleline_edit(format!("{}function {} ", parent_indent, name), formatter_info, false));
	block::format(_dbg, &cursor.node(), code, formatter_info, indent_level, commands, edits);
}
