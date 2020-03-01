use super::super::helpers::*;
use tower_lsp::lsp_types::*;
use tree_sitter::Node;

// Debugger
use std::net::TcpStream;

pub fn format(
	_dbg: &mut TcpStream,
	node: &Node,
    code: &String,
	formatter_info: &mut (u64, u64),
    _indent_level: u8,
	edits: &mut Vec<TextEdit>,
) {
    // TODO: Break string in multiple statements for HULD
    // TODO: This may be multline string.
    let string = get_node_text(node, code);
    let len: u64 = string.len() as u64;
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
        new_text: string,
    });

    formatter_info.1 += len;
}
