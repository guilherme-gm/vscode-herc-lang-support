use tower_lsp::lsp_types::*;
use tree_sitter::Node;
use super::helpers::*;
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
    let npc_type = get_next_named(&mut cursor, &code, "type").unwrap();
    let name = get_next_named(&mut cursor, &code, "name").unwrap();
    let sprite = get_next_named(&mut cursor, &code, "sprite").unwrap();
    let mut items: Vec<String> = Vec::new();
    while goto_name(&mut cursor, "item") {
        let mut item_cursor = cursor.node().walk();
        item_cursor.goto_first_child();

        let item_id = get_node_text(&item_cursor.node(), code);
        item_cursor.goto_next_sibling();
        item_cursor.goto_next_sibling();
        let item_price = get_node_text(&item_cursor.node(), code);

        items.push(format!("{}:{}", item_id, item_price));
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
            position_str, npc_type, name, sprite, items.join(",")
        ),
    });

    formatter_info.0 += 1;
    formatter_info.1 = 0;
}