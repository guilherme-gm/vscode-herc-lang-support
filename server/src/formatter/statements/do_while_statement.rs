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
	debug_!(_dbg, format!("> do_while_stmt: {:?}", node));
	let mut cursor = node.walk();
    cursor.goto_first_child(); // TODO: Maybe add handling for safety

    goto_name(&mut cursor, "body");
    let body = cursor.node();
    
    goto_name(&mut cursor, "condition");
    let condition = cursor.node();

    let parent_indent = str::repeat("\t", 1 + indent_level as usize);
    edits.push(get_singleline_edit(format!("{}do ", parent_indent), formatter_info, false));
    statements::resolve(_dbg, &body, code, formatter_info, indent_level + 1, commands, edits);
    edits.push(get_singleline_edit(String::from(" while "), formatter_info, false));
    parenthesized_expression::format(_dbg, &condition, code, formatter_info, edits);
    edits.push(get_singleline_edit(String::from(";\n"), formatter_info, true));
}
