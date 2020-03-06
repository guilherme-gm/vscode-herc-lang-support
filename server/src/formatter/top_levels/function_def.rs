use super::super::statements;
use tree_sitter::Node;

// Debugger
use std::io::prelude::*;

use super::super::script_formatter::*;

pub fn format(fmter: &mut ScriptFormatter, node: &Node) {
    fmter.info(format!("function_def: {:?}", node));

    let mut cursor = node.walk();
    cursor.goto_first_child();
    fmter.match_until_and_write_str(&mut cursor, FmtNode::Token("function"), &"function", true);
    fmter.write_edit(String::from("\t"));
    fmter.match_until_and_write_str(&mut cursor, FmtNode::Token("script"), &"script", true);
    fmter.write_edit(String::from("\t"));
    fmter.match_until_and_write_node(&mut cursor, FmtNode::Named("func_name"), true);
    fmter.write_edit(String::from("\t"));
    if fmter.match_until(&mut cursor, FmtNode::Named("body"), true) {
        statements::block::format(fmter, &cursor.node());
        cursor.goto_next_sibling();
    } else {
        panic!("function_def: Could not move to block.");
    }
}
