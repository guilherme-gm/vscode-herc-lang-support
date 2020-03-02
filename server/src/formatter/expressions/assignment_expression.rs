use super::super::helpers::*;
use super::super::expressions;
use tower_lsp::lsp_types::*;
use tree_sitter::Node;

// Debugger
use std::io::prelude::*;
use std::net::TcpStream;

fn get_opr(opr: String, formatter_info: &mut (u64, u64)) -> TextEdit {
    let opr_len = opr.len() as u64;
    let text_edit = TextEdit {
        range: Range {
            start: Position {
                line: formatter_info.0,
                character: formatter_info.1
            },
            end: Position {
                line: formatter_info.0,
                character: formatter_info.1 + opr_len
            }
        },
        new_text: opr,
    };
    formatter_info.1 += opr_len;

    text_edit
}

pub fn format(
    _dbg: &mut TcpStream,
    node: &Node,
    code: &String,
    formatter_info: &mut (u64, u64),
    edits: &mut Vec<TextEdit>,
) {
    debug_!(_dbg, format!("> assign_exp: {:?}", node));
    let mut cursor = node.walk();
    cursor.goto_first_child();

    let left = cursor.node();
    expressions::resolve(_dbg, &left, code, formatter_info, 0, edits);
    cursor.goto_next_sibling();
    
    let opr = get_node_text(&cursor.node(), code);
    edits.push(get_opr(opr, formatter_info));
    cursor.goto_next_sibling();
    
    let right = cursor.node();
    expressions::resolve(_dbg, &right, code, formatter_info, 0, edits);
}
