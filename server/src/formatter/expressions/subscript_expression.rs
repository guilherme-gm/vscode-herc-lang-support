use super::super::helpers::*;
use super::super::expressions;
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
    debug_!(_dbg, format!("> exp_subscript: {:?}", node));
    let mut cursor = node.walk();
    cursor.goto_first_child();

    let arg = get_node_text(&cursor.node(), code);
    let arg_len = arg.len() as u64;

    edits.push(TextEdit {
        range: Range {
            start: Position {
                line: formatter_info.0,
                character: formatter_info.1,
            },
            end: Position {
                line: formatter_info.0,
                character: formatter_info.1 + arg_len + 1,
            },
        },
        new_text: format!("{}[", arg),
    });
    formatter_info.1 += arg_len + 1;

    goto_name(&mut cursor, "index");
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
        new_text: String::from("]"),
    });
    formatter_info.1 += 1;
}
