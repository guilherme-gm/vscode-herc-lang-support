use super::super::script_formatter::*;
use tree_sitter::Node;

// Debugger
use std::io::prelude::*;

pub fn format(fmter: &mut ScriptFormatter, node: &Node) {
    fmter.info(format!("> end_stmt: {:?}", node));
    let mut cursor = node.walk();
    cursor.goto_first_child();
    
    fmter.match_until_and_write_str(&mut cursor, FmtNode::Token("end"), &format!("{}end", fmter.indent), true);
    fmter.match_until_and_write_node(&mut cursor, FmtNode::Token(";"), true);
}
