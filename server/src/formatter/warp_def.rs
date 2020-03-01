use tower_lsp::lsp_types::*;
use tree_sitter::Node;
use super::helpers::{ get_next_named, goto_next_named, goto_name };
use super::position;
use super::block;
use std::collections::HashMap;
use crate::script_commands::ScriptCommand;

// Debugger
use std::io::prelude::*;
use std::net::TcpStream;


pub fn format(
    _dbg: &mut TcpStream,
    node: &Node,
    code: &String,
    formatter_info: &mut (u64, u64),
    edits: &mut Vec<TextEdit>
) {
    let mut cursor = node.walk();
    cursor.goto_first_child(); // TODO: Maybe add handling for safety

    let position_str = position::format(&mut cursor.node().walk(), &code);
    let name = get_next_named(&mut cursor, &code, "name").unwrap();
    let span_x = get_next_named(&mut cursor, &code, "span_x").unwrap();
    let span_y = get_next_named(&mut cursor, &code, "span_y").unwrap();
    let to_map = get_next_named(&mut cursor, &code, "to_map").unwrap();
    let to_x = get_next_named(&mut cursor, &code, "to_x").unwrap();
    let to_y = get_next_named(&mut cursor, &code, "to_y").unwrap();
    
    edits.push(TextEdit {
        range: Range {
            start: Position {
                line: formatter_info.0,
                character: formatter_info.1,
            },
            end: Position {
                line: formatter_info.0 + 1,
                character: 0,
            },
        },
        new_text: format!(
            "{}\twarp\t{}\t{},{},{},{},{}\n",
            position_str, name, span_x, span_y, to_map, to_x, to_y
        ),
    });

    formatter_info.0 += 1;
    formatter_info.1 = 0;
}