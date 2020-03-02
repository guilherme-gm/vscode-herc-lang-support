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
	debug_!(_dbg, format!("> case_stmt: {:?}", node));
	let mut cursor = node.walk();
    cursor.goto_first_child(); // TODO: Maybe add handling for safety

    let parent_indent = str::repeat("\t", 1 + indent_level as usize);
    
    goto_next_named(&mut cursor);
    
    debug_!(_dbg, format!("{:?}", cursor.field_name()));
    if let Some(name) = cursor.field_name() {
        if name.eq_ignore_ascii_case("value") {
            let val = get_node_text(&cursor.node(), code);

            edits.push(get_singleline_edit(format!("{}case {}:\n", parent_indent, val), formatter_info, true));
            while goto_name(&mut cursor, "body") {
                statements::resolve(_dbg, &cursor.node(), code, formatter_info, indent_level + 1, commands, edits);
            }
        } else if name.eq_ignore_ascii_case("body") {
            edits.push(get_singleline_edit(format!("{}default:\n", parent_indent), formatter_info, true));

            statements::resolve(_dbg, &cursor.node(), code, formatter_info, indent_level + 1, commands, edits);

            while goto_name(&mut cursor, "body") {
                statements::resolve(_dbg, &cursor.node(), code, formatter_info, indent_level + 1, commands, edits);
            }
        }
    }
}
