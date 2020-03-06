use super::position;
use super::trigger_area;
use tree_sitter::Node;

// Debugger
use std::io::prelude::*;

use super::super::script_formatter::*;

pub fn format(fmter: &mut ScriptFormatter, node: &Node) {
    fmter.info(format!("duplicate_def: {:?}", node));

    let mut cursor = node.walk();
    cursor.goto_first_child();

    fmter.match_until(&mut cursor, FmtNode::Named("position"), true);
    position::format(fmter, &cursor.node());
    cursor.goto_next_sibling();

    fmter.write_edit(String::from("\t"));
    fmter.match_until_and_write_str(&mut cursor, FmtNode::Token("duplicate"), &"duplicate", true);
    fmter.match_until_and_write_str(&mut cursor, FmtNode::Token("("), &"(", true);
    fmter.match_until_and_write_node(&mut cursor, FmtNode::Named("src_npc"), true);
    fmter.match_until_and_write_str(&mut cursor, FmtNode::Token(")"), &")", true);
    fmter.write_edit(String::from("\t"));
    fmter.match_until_and_write_node(&mut cursor, FmtNode::Named("new_npc"), true);
    fmter.write_edit(String::from("\t"));

    fmter.match_until_one(
        &mut cursor,
        &[FmtNode::Named("span_x"), FmtNode::Named("sprite")],
        true,
    );
    if fmter.is_stop(&mut cursor, &FmtNode::Named("span_x")) {
        fmter.write_node(&mut cursor);
        fmter.match_until_and_write_str(&mut cursor, FmtNode::Token(","), &", ", true);
        fmter.match_until_and_write_node(&mut cursor, FmtNode::Named("span_y"), true);
    } else {
        // Write sprite and checks if there is more (that is, trigger)
        if fmter.write_node(&mut cursor) {
            if fmter.is_stop(&mut cursor, &FmtNode::Named("trigger")) {
                trigger_area::format(fmter, &cursor.node());
                cursor.goto_next_sibling();
            }
        }
    }

    fmter.write_edit(String::from("\n"));
}
