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
    commands: &HashMap<String, ScriptCommand>,
    edits: &mut Vec<TextEdit>
) {
    let mut cursor = node.walk();
    cursor.goto_first_child(); // TODO: Maybe add handling for safety

    let fn_name = get_next_named(&mut cursor, &code, "func_name").unwrap();

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
            "function\tscript\t{}\t{{\n",
            fn_name
        ),
    });

    formatter_info.0 += 1;
    formatter_info.1 = 0;

    if goto_name(&mut cursor, "body") {
        block::format(_dbg, &cursor.node(), code, formatter_info, 0, commands, edits);
    } else {
        panic!("Could not move to block.");
    }

}