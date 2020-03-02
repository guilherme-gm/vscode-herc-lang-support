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
    debug_!(_dbg, format!("> func_exp: {:?}", node));
    let mut cursor = node.walk();
    cursor.goto_first_child(); // TODO: Maybe add handling for safety

    let function = get_node_text(&cursor.node(), code);
    let mut params: Vec<String> = Vec::new();
    if goto_name(&mut cursor, "arguments") {
        let mut param_cursor = cursor.node().walk();
        param_cursor.goto_first_child();

        if param_cursor.node().is_named() {
            params.push(get_node_text(&param_cursor.node(), code));
        }

        while param_cursor.goto_next_sibling() {
            if !param_cursor.node().is_named() {
                continue;
            }

            // TODO: Reformat parameters
            params.push(get_node_text(&param_cursor.node(), code));
        }
    }

    let params = params.join(", ");
    let new_text = format!("{}({})", function, params);
    let new_len = new_text.len() as u64;

    edits.push(TextEdit {
        range: Range {
            start: Position {
                line: formatter_info.0,
                character: formatter_info.1,
            },
            end: Position {
                line: formatter_info.0,
                character: formatter_info.1 + new_len,
            },
        },
        new_text: new_text,
    });
    formatter_info.1 += new_len;
}
