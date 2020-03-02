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
    debug_!(_dbg, format!("> expo_update: {:?}", node));
    let mut cursor = node.walk();
    cursor.goto_first_child();

    if cursor.field_name().unwrap().eq_ignore_ascii_case("argument") {
        let arg = cursor.node();
        cursor.goto_next_sibling();
        let opr = get_node_text(&cursor.node(), code);
        
        expressions::resolve(_dbg, &arg, code, formatter_info, 0, edits);
        edits.push(get_opr(opr, formatter_info));
    } else {
        let opr = get_node_text(&cursor.node(), code);
        cursor.goto_next_sibling();
        let arg = cursor.node();
        
        edits.push(get_opr(opr, formatter_info));
        expressions::resolve(_dbg, &arg, code, formatter_info, 0, edits);
    }
}
