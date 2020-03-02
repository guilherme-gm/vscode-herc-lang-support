use super::super::helpers::*;
use tower_lsp::lsp_types::*;
use tree_sitter::Node;
use super::super::statements;
use super::super::expressions::parenthesized_expression;
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
	debug_!(_dbg, format!("> stmt_if: {:?}", node));
	let mut cursor = node.walk();
	cursor.goto_first_child(); // TODO: Maybe add handling for safety
	goto_name(&mut cursor, "condition");
	let condition = &cursor.node();
	
	goto_name(&mut cursor, "consequence");
	let consequences = &cursor.node();
	
	let alternative;
	if goto_name(&mut cursor, "alternative") {
		alternative = Some(cursor.node());
	} else {
		alternative = None;
	}
	
	debug_!(_dbg, format!(">>> cond: {:?}", condition));
	debug_!(_dbg, format!(">>> cons: {:?}", consequences));
	debug_!(_dbg, format!(">>> altr: {:?}", alternative));

	let parent_indent = str::repeat("\t", 1 + indent_level as usize);
	edits.push(get_singleline_edit(format!("{}if ", parent_indent), formatter_info, false));
	parenthesized_expression::format(_dbg, condition, code, formatter_info, edits);
	edits.push(get_singleline_edit(String::from(" "), formatter_info, false));
	statements::resolve(_dbg, consequences, code, formatter_info, indent_level + 1, commands, edits);
	if let Some(alternative) = alternative {
		edits.push(get_singleline_edit(format!("{}else ", parent_indent), formatter_info, false));
		statements::resolve(_dbg, &alternative, code, formatter_info, indent_level + 1, commands, edits);
	}
}
