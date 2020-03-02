use super::super::helpers::*;
use tower_lsp::lsp_types::*;
use tree_sitter::Node;
use super::super::statements;
use super::super::expressions;
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
	debug_!(_dbg, format!("> for_stmt: {:?}", node));
	let mut cursor = node.walk();
    cursor.goto_first_child(); // TODO: Maybe add handling for safety

    // We can't use goto_name because they are optional
    cursor.goto_next_sibling();  // (
    let mut initializer = None;
    let mut condition = None;
    let mut update = None;
    while goto_next_named(&mut cursor) {
        let name = cursor.field_name().unwrap(); // safe

        if name.eq_ignore_ascii_case("initializer") {
            initializer = Some(cursor.node());
            continue;
        }

        if name.eq_ignore_ascii_case("condition") {
            condition = Some(cursor.node());
            continue;
        }

        if name.eq_ignore_ascii_case("update") {
            update = Some(cursor.node());
            continue; // Do not break so cursor gets in the last node
        }
    }

    // cursor is now in the last node
    let stmt = cursor.node();
    
    let parent_indent = str::repeat("\t", 1 + indent_level as usize);
    edits.push(get_singleline_edit(format!("{}for (", parent_indent), formatter_info, false));
    if initializer.is_some() {
        expressions::resolve(_dbg, &initializer.unwrap(), code, formatter_info, indent_level, edits);
    }
    
    if condition.is_some() {
        edits.push(get_singleline_edit(String::from("; "), formatter_info, false));
        expressions::resolve(_dbg, &condition.unwrap(), code, formatter_info, indent_level, edits);
    } else {
        edits.push(get_singleline_edit(String::from(";"), formatter_info, false));
    }

    if update.is_some() {
        edits.push(get_singleline_edit(String::from("; "), formatter_info, false));
        expressions::resolve(_dbg, &update.unwrap(), code, formatter_info, indent_level, edits);
    } else {
        edits.push(get_singleline_edit(String::from(";"), formatter_info, false));
    }

    edits.push(get_singleline_edit(String::from(") "), formatter_info, false));
	statements::resolve(_dbg, &stmt, code, formatter_info, indent_level + 1, commands, edits);
}
