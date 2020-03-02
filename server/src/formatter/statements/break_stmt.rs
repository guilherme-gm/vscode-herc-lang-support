use tower_lsp::lsp_types::*;
use tree_sitter::Node;

use super::super::helpers::*;

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
    debug_!(_dbg, format!("> break_stmt: {:?}", node));
    
    edits.push(get_singleline_edit(format!("{}\tbreak;\n", parent_ident), formatter_info, true));
}
