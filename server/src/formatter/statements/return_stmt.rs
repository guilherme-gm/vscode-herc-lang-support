use tower_lsp::lsp_types::*;
use tree_sitter::Node;

use super::super::helpers::*;
use super::super::expressions;

// Debugger
use std::io::prelude::*;
use std::net::TcpStream;

pub fn format(
	_dbg: &mut TcpStream,
	node: &Node,
	_code: &String,
    formatter_info: &mut (u64, u64),
    indent_level: u8,
	edits: &mut Vec<TextEdit>,
) {
    let parent_ident = str::repeat("\t", indent_level as usize);
    debug_!(_dbg, format!("> return_stmt: {:?}", node));
    let mut cursor = node.walk();
    cursor.goto_first_child();

    edits.push(get_singleline_edit(format!("{}\treturn ", parent_ident), formatter_info, false));

    cursor.goto_next_sibling();
    expressions::resolve(_dbg, &cursor.node(), _code, formatter_info, indent_level, edits);

    edits.push(get_singleline_edit(String::from(";\n"), formatter_info, true));
}
