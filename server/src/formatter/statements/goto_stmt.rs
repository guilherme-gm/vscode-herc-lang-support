use tower_lsp::lsp_types::*;
use tree_sitter::Node;

use super::super::helpers::*;

// Debugger
use std::io::prelude::*;
use std::net::TcpStream;

pub fn format(
	_dbg: &mut TcpStream,
	node: &Node,
	code: &String,
    formatter_info: &mut (u64, u64),
    indent_level: u8,
	edits: &mut Vec<TextEdit>,
) {
    let parent_ident = str::repeat("\t", indent_level as usize);
    debug_!(_dbg, format!("> goto_stmt: {:?}", node));

    let mut cursor = node.walk();
    cursor.goto_first_child();
    let id = get_next_named(&mut cursor, code, "label").unwrap();
    
    edits.push(get_singleline_edit(format!("{}\tgoto {};", parent_ident, id), formatter_info, false));
}
