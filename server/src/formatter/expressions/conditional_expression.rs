use super::super::helpers::*;
use super::super::expressions;
use tower_lsp::lsp_types::*;
use tree_sitter::Node;

// Debugger
use std::net::TcpStream;

fn get_symbol(symb: String, formatter_info: &mut (u64, u64)) -> TextEdit {
    let symb_len = symb.len() as u64;
    let text_edit = TextEdit {
        range: Range {
            start: Position {
                line: formatter_info.0,
                character: formatter_info.1
            },
            end: Position {
                line: formatter_info.0,
                character: formatter_info.1 + symb_len
            }
        },
        new_text: symb,
    };
    formatter_info.1 += symb_len;

    text_edit
}

pub fn format(
    _dbg: &mut TcpStream,
    node: &Node,
    code: &String,
    formatter_info: &mut (u64, u64),
    edits: &mut Vec<TextEdit>,
) {
    let mut cursor = node.walk();
    cursor.goto_first_child();

    let condition = cursor.node();
    expressions::resolve(_dbg, &condition, code, formatter_info, 0, edits);
    cursor.goto_next_sibling();
    
    let opr = get_node_text(&cursor.node(), code);
    edits.push(get_symbol(format!(" {} ", opr), formatter_info));
    cursor.goto_next_sibling();

    let consequence = cursor.node();
    expressions::resolve(_dbg, &consequence, code, formatter_info, 0, edits);
    cursor.goto_next_sibling();

    let opr = get_node_text(&cursor.node(), code);
    edits.push(get_symbol(format!(" {} ", opr), formatter_info));
    cursor.goto_next_sibling();

    let alternative = cursor.node();
    expressions::resolve(_dbg, &alternative, code, formatter_info, 0, edits);
}
