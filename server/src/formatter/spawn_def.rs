use tower_lsp::lsp_types::*;
use tree_sitter::Node;
use super::helpers::*;
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

    let map = get_next_named(&mut cursor, &code, "map").unwrap();
    let x1 = get_next_named(&mut cursor, &code, "x1").unwrap();
    let y1 = get_next_named(&mut cursor, &code, "y1").unwrap();
    let x2 = get_next_named(&mut cursor, &code, "x2").unwrap();
    let y2 = get_next_named(&mut cursor, &code, "y2").unwrap();
    let spawn_type = get_next_named(&mut cursor, &code, "type").unwrap();
    let name = get_next_named(&mut cursor, &code, "name").unwrap();
    let id = get_next_named(&mut cursor, &code, "id").unwrap();
    let amount = get_next_named(&mut cursor, &code, "amount").unwrap();
    let delay1 = get_next_named(&mut cursor, &code, "delay1").unwrap();
    let delay2 = get_next_named(&mut cursor, &code, "delay2").unwrap();
    let event = get_next_named(&mut cursor, &code, "event").unwrap();

    let mut opt_str: String = String::from("");
    if goto_name(&mut cursor, "size") {
        let size = get_node_text(&cursor.node(), code);
        let ai = get_next_named(&mut cursor, &code, "ai").unwrap();
        opt_str = format!("{},{}", size, ai);
    }
    
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
            "{},{},{},{},{}\t{}\t{}\t{},{},{},{},{},{}\n",
            map, x1, y1, x2, y2, spawn_type, name, id, amount, delay1, delay2, event, opt_str
        ),
    });

    formatter_info.0 += 1;
    formatter_info.1 = 0;
}