use super::super::helpers::*;
use tower_lsp::lsp_types::*;
use tree_sitter::Node;

// Debugger
use std::io::prelude::*;
use std::net::TcpStream;

pub fn format(
	_dbg: &mut TcpStream,
	node: &Node,
	code: &String,
	formatter_info: &mut (u64, u64),
	edits: &mut Vec<TextEdit>,
) {
    debug_!(_dbg, format!("> exp_scop_id: {:?}", node));
    let id = get_node_text(node, code);
    let len: u64 = id.len() as u64;
	edits.push(TextEdit {
        range: Range {
            start: Position {
                line: formatter_info.0,
                character: formatter_info.1,
            },
            end: Position {
                line: formatter_info.0,
                character: formatter_info.1 + len,
            },
        },
        new_text: id,
    });

    formatter_info.1 += len;
}
