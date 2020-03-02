use tower_lsp::lsp_types::*;
use tree_sitter::Node;

// Debugger
use std::io::prelude::*;
use std::net::TcpStream;

pub fn format(
	_dbg: &mut TcpStream,
	_node: &Node,
	_code: &String,
	formatter_info: &mut (u64, u64),
	edits: &mut Vec<TextEdit>,
) {
    debug_!(_dbg, format!("> false_exp: {:?}", _node));
	edits.push(TextEdit {
        range: Range {
            start: Position {
                line: formatter_info.0,
                character: formatter_info.1,
            },
            end: Position {
                line: formatter_info.0,
                character: formatter_info.1 + 5,
            },
        },
        new_text: String::from("false"),
    });

    formatter_info.1 += 5;
}
