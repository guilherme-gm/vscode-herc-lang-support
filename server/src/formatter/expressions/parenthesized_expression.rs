use super::super::expressions;
use tower_lsp::lsp_types::*;
use tree_sitter::Node;

// Debugger
use std::net::TcpStream;

pub fn format(
    _dbg: &mut TcpStream,
    node: &Node,
    code: &String,
    formatter_info: &mut (u64, u64),
    edits: &mut Vec<TextEdit>,
) {
    edits.push(TextEdit {
        range: Range {
            start: Position {
                line: formatter_info.0,
                character: formatter_info.1,
            },
            end: Position {
                line: formatter_info.0,
                character: formatter_info.1 + 1,
            },
        },
        new_text: String::from("("),
    });
    formatter_info.1 += 1;

    let mut cursor = node.walk();
    cursor.goto_first_child();
    cursor.goto_next_sibling();

    expressions::resolve(_dbg, &cursor.node(), code, formatter_info, 0, edits);

    edits.push(TextEdit {
        range: Range {
            start: Position {
                line: formatter_info.0,
                character: formatter_info.1,
            },
            end: Position {
                line: formatter_info.0,
                character: formatter_info.1 + 1,
            },
        },
        new_text: String::from(")"),
    });
    formatter_info.1 += 1;
}
