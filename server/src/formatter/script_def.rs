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

    let position_str = position::format(&mut cursor.node().walk(), &code);
    let npc_type = get_next_named(&mut cursor, &code, "type").unwrap();
    let name = get_next_named(&mut cursor, &code, "name").unwrap();
    let sprite = get_next_named(&mut cursor, &code, "sprite").unwrap();
    let mut trigger: String = String::from("");
    if goto_next_named(&mut cursor) && cursor.field_name().is_some() && !cursor.field_name().unwrap().eq_ignore_ascii_case("body") {
        let mut trigger_cursor = cursor.node().walk();
        trigger_cursor.goto_first_child();
        let span_x = get_next_named(&mut trigger_cursor, &code, "span_x").unwrap();
        let span_y = get_next_named(&mut trigger_cursor, &code, "span_y").unwrap();
        trigger = format!(" {}, {},", span_x, span_y);
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
            "{}\t{}\t{}\t{},{}{{\n",
            position_str, npc_type, name, sprite, trigger
        ),
    });

    formatter_info.0 += 1;
    formatter_info.1 = 0;

    if cursor.field_name().is_some() && cursor.field_name().unwrap().eq_ignore_ascii_case("body") || goto_name(&mut cursor, "body") {
        block::format(_dbg, &cursor.node(), code, formatter_info, 0, commands, edits);
    } else {
        panic!("Could not move to block.");
    }

}