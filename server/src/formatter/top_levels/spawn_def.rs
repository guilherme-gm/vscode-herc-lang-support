use tree_sitter::Node;

// Debugger
use std::io::prelude::*;

use super::super::script_formatter::*;

pub fn format(fmter: &mut ScriptFormatter, node: &Node) {
    fmter.info(format!("spawn_def: {:?}", node));

    let mut cursor = node.walk();
    cursor.goto_first_child();
    fmter.match_until_and_write_node(&mut cursor, FmtNode::Named("map"), true);
    fmter.match_until_and_write_str(&mut cursor, FmtNode::Token(","), &", ", true);
    fmter.match_until_and_write_node(&mut cursor, FmtNode::Named("x1"), true);
    fmter.match_until_and_write_str(&mut cursor, FmtNode::Token(","), &", ", true);
    fmter.match_until_and_write_node(&mut cursor, FmtNode::Named("y1"), true);
    fmter.match_until_and_write_str(&mut cursor, FmtNode::Token(","), &", ", true);
    fmter.match_until_and_write_node(&mut cursor, FmtNode::Named("x2"), true);
    fmter.match_until_and_write_str(&mut cursor, FmtNode::Token(","), &", ", true);
    fmter.match_until_and_write_node(&mut cursor, FmtNode::Named("y2"), true);
    fmter.write_edit(String::from("\t"));
    fmter.match_until_and_write_node(&mut cursor, FmtNode::Named("type"), true);
    fmter.write_edit(String::from("\t"));
    fmter.match_until_and_write_node(&mut cursor, FmtNode::Named("name"), true);
    fmter.write_edit(String::from("\t"));
    fmter.match_until_and_write_node(&mut cursor, FmtNode::Named("id"), true);
    fmter.match_until_and_write_str(&mut cursor, FmtNode::Token(","), &", ", true);
    fmter.match_until_and_write_node(&mut cursor, FmtNode::Named("amount"), true);
    fmter.match_until_and_write_str(&mut cursor, FmtNode::Token(","), &", ", true);
    fmter.match_until_and_write_node(&mut cursor, FmtNode::Named("delay1"), true);
    fmter.match_until_and_write_str(&mut cursor, FmtNode::Token(","), &", ", true);
    fmter.match_until_and_write_node(&mut cursor, FmtNode::Named("delay2"), true);
    fmter.match_until_and_write_str(&mut cursor, FmtNode::Token(","), &", ", true);
    let has_more = fmter.match_until_and_write_node(&mut cursor, FmtNode::Named("event"), true);

    if has_more {
        fmter.match_until_and_write_str(&mut cursor, FmtNode::Token(","), &", ", true);
        fmter.match_until_and_write_node(&mut cursor, FmtNode::Named("size"), true);
        fmter.match_until_and_write_str(&mut cursor, FmtNode::Token(","), &", ", true);
        fmter.match_until_and_write_node(&mut cursor, FmtNode::Named("ai"), true);
    }

    fmter.write_edit(String::from("\n"));
}
