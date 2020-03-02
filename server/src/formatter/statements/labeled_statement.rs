use super::super::helpers::*;
use tower_lsp::lsp_types::*;
use tree_sitter::Node;
use super::super::statements;
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
	debug_!(_dbg, format!("> label_stmt: {:?}", node));
	let mut cursor = node.walk();
    cursor.goto_first_child(); // TODO: Maybe add handling for safety

    // let lbl = get_next_named(&mut cursor, code, "label").unwrap();
    let lbl = get_node_text(&cursor.node(), code);
    edits.push(get_singleline_edit(format!("{}:\n", lbl), formatter_info, true));
    
    goto_name(&mut cursor, "body");
    let body = cursor.node();

    statements::resolve(_dbg, &body, code, formatter_info, indent_level, commands, edits);
}
