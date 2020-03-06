use super::super::script_formatter::*;
use tree_sitter::Node;

// Debugger
use std::io::prelude::*;

pub fn format(fmter: &mut ScriptFormatter, node: &Node) {
    // TODO: Break string in multiple statements for HULD
    // TODO: This may be multline string.

    fmter.info(format!("> exp_string: {:?}", node));
    let mut cursor = node.walk();
    cursor.goto_first_child();

    if fmter.match_until_and_write_node(&mut cursor, FmtNode::Named("line"), true) {
        while fmter.match_until_and_write_node(&mut cursor, FmtNode::Named("line"), false) {
            /* do nothing */
        }
    }
}
