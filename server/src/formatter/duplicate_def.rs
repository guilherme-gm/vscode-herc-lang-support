use tower_lsp::lsp_types::*;
use tree_sitter::Node;
use super::helpers::{ get_next_named, goto_next_named, get_node_text };
use super::position;

// Debugger
use std::io::prelude::*;
use std::net::TcpStream;


pub fn format(
    _dbg: &mut TcpStream,
    node: &Node,
    code: &String,
    formatter_info: &mut (u64, u64),
) -> TextEdit {
    let mut cursor = node.walk();
    cursor.goto_first_child(); // TODO: Maybe add handling for safety

    let position_str = position::format(&mut cursor.node().walk(), &code);
    let src_npc = get_next_named(&mut cursor, &code, "src_npc").unwrap();
    let new_npc = get_next_named(&mut cursor, &code, "new_npc").unwrap();

    // TODO: Improve this
    let mut found: u8 = 0;
    while goto_next_named(&mut cursor) {
        if let Some(name) = cursor.field_name() {
            if name.eq_ignore_ascii_case("span_x") {
                found = 1;
                break;
            } else if name.eq_ignore_ascii_case("sprite") {
                found = 2;
                break;
            }
        }
    }

    let mut part3: String = String::from("");
    if found == 0 {
        panic!("Invalid duplicate");
    } else if found == 1 {
        let span_x = get_node_text(&cursor.node(), &code);
        let span_y = get_next_named(&mut cursor, &code, "span_y").unwrap();
        part3 = format!("{}, {}", span_x, span_y);
    } else if found == 2 {
        let sprite = get_node_text(&cursor.node(), &code);
        if goto_next_named(&mut cursor) {
            let mut trigger_cursor = cursor.node().walk();
            trigger_cursor.goto_first_child();
            let span_x = get_next_named(&mut trigger_cursor, &code, "span_x").unwrap();
            let span_y = get_next_named(&mut trigger_cursor, &code, "span_y").unwrap();
            part3 = format!("{}, {}, {}", sprite, span_x, span_y);
        } else {
            part3 = format!("{}", sprite);
        }
    }

    let text_edit = TextEdit {
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
        // <map>,<x>,<y>,<dir>%TAB%duplicate(<Name>)%TAB%<Name>%TAB%<sprite>,<spanx>,<spany>
        new_text: format!(
            "{}\tduplicate({})\t{}\t{}\n",
            position_str, src_npc, new_npc, part3
        ),
    };

    formatter_info.0 += 1;
    formatter_info.1 = 0;

    text_edit
}
