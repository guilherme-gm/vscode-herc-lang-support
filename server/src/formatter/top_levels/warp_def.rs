use super::position;
use tree_sitter::Node;

// Debugger
use std::io::prelude::*;

use super::super::script_formatter::*;

pub fn format(fmter: &mut ScriptFormatter, node: &Node) {
    fmter.info(format!("warp_def: {:?}", node));

    let mut cursor = node.walk();
    cursor.goto_first_child();

    fmter.match_until(&mut cursor, FmtNode::Named("position"), true);
    position::format(fmter, &cursor.node());
    cursor.goto_next_sibling();
    fmter.write_edit(String::from("\t"));
    fmter.match_until_and_write_node(&mut cursor, FmtNode::Token("warp"), true);
    fmter.write_edit(String::from("\t"));
    fmter.match_until_and_write_node(&mut cursor, FmtNode::Named("name"), true);
    fmter.write_edit(String::from("\t"));
    fmter.match_until_and_write_node(&mut cursor, FmtNode::Named("span_x"), true);
    fmter.match_until_and_write_str(&mut cursor, FmtNode::Token(","), &", ", true);
    fmter.match_until_and_write_node(&mut cursor, FmtNode::Named("span_y"), true);
    fmter.match_until_and_write_str(&mut cursor, FmtNode::Token(","), &", ", true);
    fmter.match_until_and_write_node(&mut cursor, FmtNode::Named("to_map"), true);
    fmter.match_until_and_write_str(&mut cursor, FmtNode::Token(","), &", ", true);
    fmter.match_until_and_write_node(&mut cursor, FmtNode::Named("to_x"), true);
    fmter.match_until_and_write_str(&mut cursor, FmtNode::Token(","), &", ", true);
    fmter.match_until_and_write_node(&mut cursor, FmtNode::Named("to_y"), true);

    fmter.write_edit(String::from("\n"));
}
